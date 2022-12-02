/*!
asiddhavat
==========
(6.4.22 - 6.4.175 [end])

Rules in the *asiddhavat* rules do not interfere with each other. That is, if
a rule A would ordinary block a rule B, both are allowed to apply if they are
defined within this section.

*asiddhavat* rules are within the scope of the *aNgasya* adhikAra. For details,
see the `angasya` module.
*/

use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::{Term, TermView};
use lazy_static::lazy_static;

lazy_static! {
    // The name has two Is for readability.
    static ref II: SoundSet = s("i");
    static ref UU: SoundSet = s("u");
    static ref I_U: SoundSet = s("i u");
    static ref AC: SoundSet = s("ac");
    static ref HAL: SoundSet = s("hal");
    static ref JHAL: SoundSet = s("Jal");
    static ref MAHAPRANA: SoundSet = s("K G C J W Q T D P B");
}

fn is_knit(n: &TermView) -> bool {
    n.any(&[T::kit, T::Nit])
}

/// Returns whether the given slice has multiple vowels.
fn is_anekac(p: &Prakriya, i: usize) -> bool {
    let mut num_ac = 0_u8;
    for t in p.terms()[..i].iter().rev() {
        for c in t.text.chars().rev() {
            if AC.contains_char(c) {
                num_ac += 1;
                if num_ac >= 2 {
                    return true;
                }
            }
        }
    }
    false
}

/// Returns whether the given slice ends in a samyoga.
fn is_samyogapurva(p: &Prakriya, i: usize) -> bool {
    let mut num_hal = 0_u8;
    for t in p.terms()[..i].iter().rev() {
        for c in t.text.chars().rev() {
            if HAL.contains_char(c) {
                num_hal += 1;
                if num_hal >= 2 {
                    return true;
                }
            } else {
                return false;
            }
        }
    }
    false
}

/// Runs rules conditioned on a following knit ArdhadhAtuka suffix.
///
/// (6.4.63 - 6.4.69)
fn run_kniti_ardhadhatuka(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let n = p.view(i + 1)?;

    let aat = dhatu.has_antya('A');
    let kniti_ardha = n.any(&[T::kit, T::Nit]) && n.has_tag(T::Ardhadhatuka);

    if kniti_ardha && dhatu.has_u("dI\\N") && n.has_adi(&*AC) {
        p.op("6.4.63", |p| op::insert_agama_after(p, i, "yu~w"));
        // No change to `n` index (`i + 1`) needed since `yu~w` is an agama and will will be
        // included in `n`.
    } else if aat && n.has_adi(&*AC) && (kniti_ardha || f::is_it_agama(n.first()?)) {
        p.op_term("6.4.64", i, op::antya(""));
    } else if aat && kniti_ardha {
        let ghu_ma = dhatu.has_tag(T::Ghu)
            || dhatu.has_text_in(&["mA", "sTA", "gA", "sA"])
            || dhatu.has_u("o~hA\\k")
            || (dhatu.has_u("pA\\") && dhatu.has_gana(1));
        if n.has_u("yat") {
            p.op_term("6.4.65", i, op::antya("I"));
        } else if n.has_adi(&*HAL) && ghu_ma {
            if n.has_lakshana("li~N") {
                p.op_term("6.4.67", i, op::antya("e"));
            } else {
                p.op_term("6.4.66", i, op::antya("I"));
            }
        } else if f::is_samyogadi(dhatu) {
            // HACK: skip dhatus with agama. `k` indicates a following agama.
            let next = p.get(i + 1)?;
            if next.all(&[T::Agama, T::kit]) {
                return None;
            }

            if n.has_u("lyap") {
                p.step("6.4.69");
            } else if n.has_lakshana("li~N") {
                p.op_optional("6.4.68", op::t(i, op::antya("e")));
            }
        }
    }

    Some(())
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
fn try_run_kniti(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.any(&[T::kit, T::Nit]) {
        return None;
    }

    let next_is_hi = n.first()?.has_text("hi");

    if anga.has_text_in(&["gam", "han", "jan", "Kan", "Gas"]) && n.has_adi(&*AC) && !n.has_u("aN") {
        p.op_term("6.4.98", i, op::upadha(""));
    } else if (anga.has_text("hu") || anga.has_antya(&*JHAL)) && next_is_hi {
        p.op_term("6.4.101", n.start(), op::text("Di"));
    } else if anga.has_u("ciR") {
        p.op_term("6.4.104", n.start(), op::luk);
    } else if anga.has_antya('a') && n.first()?.has_text("hi") {
        p.op_term("6.4.105", n.start(), op::luk);
    } else if anga.has_antya('u') && anga.has_tag(T::Pratyaya) {
        let dhatu = p.get(i - 1)?;
        let n_is_mv = n.has_adi('m') || n.has_adi('v');

        if !dhatu.has_antya(&*HAL) && next_is_hi {
            p.op_term("6.4.106", n.start(), op::luk);
        } else if dhatu.has_text_in(&["kar", "kur"]) {
            if n_is_mv {
                p.op_term("6.4.108", i, op::luk);
            } else if n.has_adi('y') {
                p.op_term("6.4.109", i, op::luk);
            }
        } else if n_is_mv && !is_samyogapurva(p, i) {
            p.op_optional("6.4.107", op::t(i, op::antya("")));
        }
    }

    try_run_kniti_sarvadhatuke(p, i);

    Some(())
}

fn try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !(anga.has_u("SnA") || anga.has_tag(T::Abhyasta)) {
        return None;
    }

    let n_is_haladi = n.has_adi(&*HAL);
    if anga.has_text("daridrA") && n_is_haladi {
        p.op_term("6.4.114", i, op::antya("i"));
    } else if anga.has_u("YiBI\\") && n_is_haladi {
        p.op_optional("6.4.115", op::t(i, op::antya("i")));
    } else if anga.has_antya('A') {
        if anga.has_u("o~hA\\k") && n_is_haladi {
            if n.has_adi('y') {
                p.op_term("6.4.118", i, op::antya(""));
            } else {
                let mut run_116 = true;
                if n.first()?.has_text("hi") {
                    // Run 6.4.116 only if 6.4.117 was not run.
                    run_116 = !p.op_optional("6.4.117", op::t(i, op::antya("A")));
                }
                if run_116 {
                    p.op_optional("6.4.116", op::t(i, op::antya("i")));
                }
            }
        } else if !anga.has_tag(T::Ghu) && n_is_haladi {
            p.op_term("6.4.113", i, op::antya("I"));
        } else {
            p.op_term("6.4.112", i, op::antya(""));
        }
    }

    Some(())
}

fn try_run_kniti_sarvadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.has_tag(T::Sarvadhatuka) {
        return None;
    }

    // Must come before 6.4.111.
    if (anga.has_u("asa~") || anga.has_tag(T::Ghu)) && n.has_u("hi") {
        p.op("6.4.119", |p| {
            if let Some(a) = p.find_first(T::Abhyasa) {
                p.set(a, op::text(""));
            }
            p.set(i, op::antya("e"));
        });
    }

    let anga = p.get(i)?;
    if anga.has_u("Snam") {
        p.op_term("6.4.111", i, |t| {
            t.text = t.text.replace("na", "n");
        });
    } else if anga.has_u("asa~") {
        p.op_term("6.4.111", i, |t| t.text = t.text.replace('a', ""));
    } else {
        try_run_kniti_sarvadhatuke_for_shna_and_abhyasta(p, i);
    }

    Some(())
}

/// Run rules that replace the dhatu's vowel with e and apply abhyasa-lopa.
/// Example: `la + laB + e` -> `leBe`
fn try_et_adesha_and_abhyasa_lopa_for_it(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if !dhatu.all(&[T::Dhatu, T::Abhyasta]) {
        return None;
    }
    let abhyasa = p.get(i - 1)?;
    if !abhyasa.has_tag(T::Abhyasa) {
        return None;
    }
    let n = p.view(i + 1)?;

    let kniti = n.any(&[T::kit, T::Nit]);
    let thali_seti = n.get(0)?.has_u("iw") && n.get(1)?.has_u("Tal");
    if !(kniti || thali_seti) {
        return None;
    }

    let op_et_abhyasa_lopa = |p: &mut Prakriya| {
        p.set(i, op::upadha("e"));
        p.set(i - 1, op::lopa);
    };

    if dhatu.text == "daB" && dhatu.has_u("danBu~") {
        p.op("6.4.120", op_et_abhyasa_lopa);
    } else if dhatu.has_u("tF") || dhatu.has_text_in(&["Pal", "Baj", "trap"]) {
        p.op("6.4.122", op_et_abhyasa_lopa);
    } else if dhatu.has_text("SraT") && dhatu.has_u("SranTa~") {
        p.op("6.4.122.v1", op_et_abhyasa_lopa);
    } else if dhatu.has_text("graT") {
        // TODO: attested, but can't find the rule for it.
        p.op("???", op_et_abhyasa_lopa);
    } else if dhatu.has_text("rAD") {
        p.op_optional("6.4.123", op_et_abhyasa_lopa);
    } else if dhatu.has_u("jF") || dhatu.has_text_in(&["Bram", "tras"]) {
        p.op_optional("6.4.124", op_et_abhyasa_lopa);
    } else if dhatu.has_u_in(gana::PHAN_ADI) {
        p.op_optional("6.4.125", op_et_abhyasa_lopa);
    } else if dhatu.has_text_in(&["Sas", "dad"]) || dhatu.has_adi('v') || dhatu.has_tag(T::FlagGuna)
    {
        // No change.
        p.step("6.4.126")
    } else {
        let is_eka_hal_madhya =
            dhatu.text.len() == 3 && dhatu.has_adi(&*HAL) && dhatu.has_antya(&*HAL);
        let is_a = dhatu.has_upadha('a');
        let is_lit = n.has_lakshana("li~w");
        // Aspirated consonants become usaspirated in the tripAdi, which hasn't run
        // yet at this stage in the derivation. So, also "look ahead" and check for
        // aspirated consonants.
        let is_anadeshadi = abhyasa.adi() == dhatu.adi() && !abhyasa.has_adi(&*MAHAPRANA);

        if is_eka_hal_madhya && is_a && is_lit && is_anadeshadi {
            if kniti {
                // `la laB e` -> `leBe`
                p.op("6.4.120", op_et_abhyasa_lopa);
            } else {
                // `Sa Sak i Ta` -> `SekiTa`
                p.op("6.4.121", op_et_abhyasa_lopa);
            }
        }
    }

    Some(())
}

/// Runs rules conditioned on a following ardhadhatuka suffix.
///
/// (6.4.46 - 6.4.70)
fn try_ardhadhatuke(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    if !n.has_tag(T::Ardhadhatuka) {
        return None;
    }

    // HACK to avoid abhyasa-at-lopa
    if anga.has_tag(T::Abhyasa) {
        return None;
    }

    if anga.has_text("Brasj") {
        p.op_optional("6.4.47", op::t(i, op::text("Barj")));
    } else if anga.has_antya('a') {
        p.op("6.4.48", |p| {
            p.set(i, op::antya(""));
            p.set(i, op::add_tag(T::FlagAtLopa));
            p.add_tag(T::FlagAtLopa);
        });
    }

    Some(())
}

/*
fn run_dirgha(p: Prakriya):
    """6.4.2 - 6.4.19"""

    sup = p.terms[-1]
    if not sup.all(T.SUP):
        return
    anga = p.terms[-2]

    has_num = False
    if anga.u == "nu~w":
        anga = p.terms[-3]
        has_num = True

    if sup.text == "Am" and has_num:
        if anga.text in {"tisf", "catasf"}:
            p.step("6.4.3")
        } else if  anga.text == "nf":
            op.optional(op.antya, "6.4.4", p, anga, sounds.dirgha(anga.antya))
        } else if  anga.antya == "n":
            op.upadha("6.4.5", p, anga, sounds.dirgha(anga.upadha))
        } else if  anga.antya in s("ac"):
            op.antya("6.4.2", p, anga, sounds.dirgha(anga.antya))

    } else if  sup.any(T.SARVANAMASTHANA) and not sup.any(T.SAMBUDDHI):
        tr_exclude = {"pitf", "pitar", "jAmAtf", "jAmAtar", "BrAtf", "BrAtar"}
        if anga.antya == "n":
            op.upadha("6.4.8", p, anga, sounds.dirgha(anga.upadha))
        // TODO: restrict
        } else if  (
            anga.antya == "f" or anga.text.endswith("ar")
        ) and anga.text not in tr_exclude:
            op.upadha("6.4.11", p, anga, sounds.dirgha(anga.upadha))
*/

fn try_upadha_nalopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    if anga.has_tag(T::Snam) && anga.has_upadha('n') {
        p.op_term("6.4.23", i, op::upadha(""));
    }

    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    let anidit_hal = !anga.has_tag(T::idit) && anga.has_antya(&*HAL);
    let is_kniti = n.any(&[T::kit, T::Nit]);

    if anidit_hal && is_kniti && anga.has_upadha('n') {
        let mut can_run = true;
        // ancu gati-pUjanayoH
        if anga.has_u("ancu~") {
            let code = "6.4.30";
            if p.is_allowed(code) {
                p.step(code);
            } else {
                p.decline(code);
                can_run = false;
            }
        }
        // TODO: 6.4.31 etc.
        if can_run {
            p.op_term("6.4.24", i, op::upadha(""));
        }
    } else if anga.has_text_in(&["danS", "sanj", "svanj"]) && n.has_u("Sap") {
        p.op_term("6.4.25", i, op::upadha(""));
    } else if anga.has_text("ranj") {
        if n.has_u("Sap") {
            p.op_term("6.4.26", i, op::upadha(""));
        } else if n.has_u("GaY") {
            p.op_optional("6.4.27", op::t(i, op::upadha("")));
        }
    } else if anga.has_text("syad") && n.has_u("GaY") {
        p.op_optional("6.4.28", op::t(i, op::upadha("")));
    } else if anga.has_text("SAs") && is_kniti && (n.has_u("aN") || n.has_adi(&*HAL)) {
        p.op_term("6.4.34", i, op::upadha("i"));
    } else if anga.has_text("SAs") && n.last()?.has_text("hi") {
        p.op_term("6.4.35", i, op::text("SA"));
    }

    Some(())
}

/// Runs rules that delete the final n of a term.
///
/// (6.4.36 - 6.4.44)
/// TODO: 6.4.41
fn try_antya_nalopa(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i+1)?;

    if !(anga.has_antya('n') || anga.has_antya('m')) {
        return None;
    }

    let is_anudatta = anga.has_tag(T::Anudatta);
    let is_tanadi = anga.has_u_in(gana::TAN_ADI);

    let jhali_kniti = n.has_adi(&*JHAL) && is_knit(&n);

    if anga.has_text("han") && n.last()?.has_text("hi") {
        p.op_term("6.4.36", i, op::text("ja"));
    } else if anga.has_text("gam") && n.has_u("kvip") {
        // TODO: other kvi-pratyayas?
        p.op_term("6.4.40", i, op::antya(""));
    } else if anga.has_text_in(&["jan", "san", "Kan"]) && (jhali_kniti || n.has_u("san")) {
        if n.has_adi('y') {
            // Syan-pratyaya excluded by "jhali kniti".
            p.op_optional("6.4.43", op::t(i, op::antya("A")));
        } else {
            p.op_term("6.4.42", i, op::antya("A"));
        }
    } else if anga.has_text("tan") && n.has_u("yak") {
        p.op_optional("6.4.44", op::t(i, op::antya("A")));
    } else if anga.has_text("san") && n.has_u("ktic") {
        let used = p.op_optional("6.4.45.b", op::t(i, op::antya("")));
        if !used {
            p.op_optional("6.4.45.a", op::t(i, op::antya("A")));
        }
    } else if is_anudatta || is_tanadi || anga.has_text("van") && jhali_kniti {
        // General case
        //
        if n.has_u("lyap") {
            p.op_optional("6.4.38", op::t(i, op::antya("")));
        } else if n.has_u("ktic") {
            // TODO: also prevent 6.4.15;
            p.step("6.4.39");
        } else {
            p.op_term("6.4.37", i, op::antya(""));
        }
    }

    Some(())
}

fn try_add_a_agama(p: &mut Prakriya, i: usize) {
    if p.find_last(T::Dhatu).is_none() {
        return;
    };
    let i_tin = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    if !p.has(i_tin, f::lakshana_in(&["lu~N", "la~N", "lf~N"])) {
        return;
    }
    // Dhatu may be multi-part, so insert before abhyasa.
    // But abhyasa may follow main dhatu (e.g. undidizati) --
    // So, use the first match we find.
    let i_start = match p.find_first_where(|t| t.any(&[T::Abhyasa, T::Dhatu])) {
        Some(i) => i,
        None => return,
    };

    // Agama already added in a previous iteration, so return.
    // (To prevent infinite loops)
    if i_start > 0 && p.has(i_start - 1, f::tag(T::Agama)) {
        return;
    }

    if p.has(i, f::adi("ac")) {
        let agama = Term::make_agama("Aw");
        p.insert_before(i, agama);
        p.step("6.4.72");
        it_samjna::run(p, i).unwrap();
    } else {
        let agama = Term::make_agama("aw");
        p.insert_before(i, agama);
        p.step("6.4.71");
        it_samjna::run(p, i).unwrap();
    }
}

pub fn run_before_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    try_upadha_nalopa(p, i);
    try_antya_nalopa(p, i);
    try_add_a_agama(p, i);
    try_ardhadhatuke(p, i);

    // Must run before guNa.
    let n = p.view(i + 1)?;
    if p.has(i, f::text("BU")) && n.has_lakshana_in(&["lu~N", "li~w"]) {
        op::append_agama("6.4.88", p, i, "vu~k");
    }

    /*
    if c.u == "ciR" and n.text == "ta":
        op.luk("6.4.104", p, n.terms[0])

    // 6.4.114 has a vArttika for ArdhadhAtuke:
    } else if c.u == "daridrA" and n.any(T.ARDHADHATUKA):
        if p.terms[-1].all("lu~N"):
            if p.allow("6.4.114.v2"):
                p.step("6.4.114.v2")
                return
            else:
                p.decline("6.4.114.v2")

        // Should replace just the last sound, but sak-Agama causes issues
        // here.
        // TODO: what is the correct prakriya here?
        op.text("6.4.114.v1", p, c, "daridr")
    */

    Some(())
}

// Runs rules that are conditioned on an anga ending in an "i" or "v" sound.
//
// (6.4.77 - 6.4.100)
fn run_for_final_i_or_u(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !anga.has_antya(&*I_U) || !n.has_adi(&*AC) {
        return None;
    }

    let to_iy_uv = |p: &mut Prakriya, i| {
        if p.has(i, |t| t.has_antya(&*II)) {
            p.set(i, op::antya("iy"));
        } else {
            p.set(i, op::antya("uv"));
        }
    };

    let is_asamyogapurva = !is_samyogapurva(p, i);
    if anga.has_text("strI") {
        if n.last()?.has_u_in(&["am", "Sas"]) {
            p.op_optional("6.4.80", op::t(i, op::antya("iy")));
        } else {
            p.op_term("6.4.79", i, op::antya("iy"));
        }
    } else if anga.has_u("i\\R") {
        p.op_term("6.4.81", i, op::antya("y"));
    } else if anga.has_antya(&*II) && is_anekac(p, i) && is_asamyogapurva {
        if anga.has_text("suDI") {
            p.step("6.4.85");
        } else {
            p.op_term("6.4.82", i, op::antya("y"));
        }
    } else if anga.has_antya(&*UU) && n.has_tag(T::Sup) && is_anekac(p, i) && is_asamyogapurva {
        if anga.has_text("BU") {
            p.step("6.4.85");
        } else {
            p.op_term("6.4.83", i, op::antya("v"));
        }
    } else if anga.has_text("varzABU") {
        p.op_term("6.4.84", i, op::antya("v"));
    } else if anga.has_u_in(&["hu\\", "Snu"]) && n.has_tag(T::Sarvadhatuka) && is_asamyogapurva {
        p.op_term("6.4.87", i, op::antya("v"));
    } else if anga.has_tag(T::Dhatu) || anga.has_u("Snu") || anga.has_text("BrU") {
        p.op("6.4.77", |p| to_iy_uv(p, i));
    } else {
        // TODO 6.4.78
    }

    Some(())
}

/// Runs asiddhavat rules that alter a Ri suffix.
pub fn run_for_ni(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_where(|t| t.has_u_in(&["Ric", "RiN"]))?;
    let c = p.get(i)?;
    let n = p.view(i + 1)?;

    /*
        if (
            c.u in ("Ric", "RiN")
            and not f.is_it_agama(n.terms[0])
            and n.all(T.ARDHADHATUKA)
        ):
            n_text = n.terms[0].text
            if n_text in {"Am", "anta", "Alu", "Ayya", "itnu", "iznu"}:
                op.antya("6.4.55", p, c, "ay")
            else:
                // Apply ac_sandhi before lopa, since later rules depend on this
                // being done (e.g. cayyAt)
                ac_sandhi.general_vowel_sandhi(p, p.terms[index - 1 : index + 1])
                op.antya("6.4.51", p, c, "")
    */

    if c.has_tag(T::mit) && n.has_u("Ric") && c.has_upadha(&*AC) {
        if let Some(sub) = al::to_hrasva(c.upadha()?) {
            p.op_term("6.4.92", i, op::upadha(&sub.to_string()));
        }
    }

    Some(())
}

pub fn run_after_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    run_kniti_ardhadhatuka(p, i);
    run_for_final_i_or_u(p, i);
    try_run_kniti(p, i);

    /*
        // TODO: fails kniti check because this depends on the last affix, and
        // term view includes only "u" here. So the rule is awkwardly placed
        // here.
        last = p.terms[-1]
        sarva_kniti = last.all(T.SARVADHATUKA) and last.any("k", "N")
        if c.u == "qukf\\Y" and c.text == "kar" and n.adi == "u" and sarva_kniti:
            c.text = "kur"
            p.step("6.4.110")

    */
    try_et_adesha_and_abhyasa_lopa_for_it(p, i);

    let n = p.view(i + 1)?;
    if n.has_tag(T::qit) {
        p.op_term("6.4.143", i, op::ti(""));
    }

    Some(())
}
