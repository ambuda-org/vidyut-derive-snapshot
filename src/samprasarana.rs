use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::operators as op;
/// Applies samprasarana changes as needed.
///
/// Order of operations:
/// - Must follow atidesha so that suffixes have the kit/Nit annotations necessary to cause
///   samprasanara.
use crate::prakriya::{Prakriya, Rule};
use crate::term::Term;

fn is_vaci_svapi(t: &Term) -> bool {
    t.has_tag(T::Dhatu)
        && (t.has_u_in(&["va\\ca~", "Yizva\\pa~"])
            || t.has_u_in(gana::YAJ_ADI)
            || t.has_u("va\\ci~"))
}

fn is_grahi_jya(t: &Term) -> bool {
    t.has_tag(T::Dhatu)
        && t.has_u_in(&[
            "graha~^",
            "jyA\\",
            // vayi~ replaces ve\\Y in 2.4.41
            "vayi~",
            "vya\\Da~",
            "vaSa~",
            "vyaca~",
            "o~vrascU~",
            "pra\\Ca~",
            "Bra\\sja~^",
            // not sure how to handle "vay" root
            "vaya~\\",
        ])
}

/// Runs a hacky version of samprasarana that runs 6.1.108 (samprasAraNAcca) immediately.
///
/// TODO: properly annotade 6.1.108 and related rules here.
fn try_vaci_svapi_samprasarana(rule: Rule, p: &mut Prakriya, i: usize) {
    let before = &[
        "vac", "svap", "yaj", "vap", "vah", "vas", "ve", "vye", "hve", "vad", "Svi",
    ];
    let after = &[
        "uc", "sup", "ij", "up", "uh", "us", "u", "vI", "hU", "ud", "SU",
    ];
    let text = &p.terms()[i].text;
    if let Some(j) = before.iter().position(|x| x == text) {
        p.op_term(rule, i, op::text(after[j]));
    }
}

fn try_grahi_jya_samprasarana(rule: Rule, p: &mut Prakriya, i: usize) {
    let before = &[
        "grah", "jyA", "vay", "vyaD", "vaS", "vyac", "vrasc", "praC", "Brasj",
    ];
    let after = &["gfh", "ji", "uy", "viD", "uS", "vic", "vfSc", "pfC", "Bfsj"];

    let text = &p.terms()[i].text;
    if let Some(j) = before.iter().position(|x| x == text) {
        p.op_term(rule, i, op::text(after[j]));
    }
}

pub fn run_for_dhatu(p: &mut Prakriya) {
    p.step("for dhatu");
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };
    let n = match p.view(i + 1) {
        Some(n) => n,
        None => return,
    };

    let dhatu = &p.terms()[i];
    if is_vaci_svapi(dhatu) && n.has_tag(T::kit) {
        if dhatu.has_u("ve\\Y") && n.has_lakshana("li~w") {
            p.step("6.1.40");
        } else {
            try_vaci_svapi_samprasarana("6.1.15", p, i);
        }
    } else if is_grahi_jya(dhatu) && n.any(&[T::kit, T::Nit]) {
        try_grahi_jya_samprasarana("6.1.16", p, i);
        if p.has(i, |t| t.text == "uy" && t.has_u("vayi~")) {
            p.op_optional("6.1.39", op::t(i, op::text("uv")));
        }
    }

    let dhatu = &p.terms()[i];
    let n = p.view(i + 1).unwrap();
    let next_is_lit = n.has_lakshana("li~w");
    let next_is_lit_or_yan = next_is_lit || n.has_u("yaN");
    let next_will_be_abhyasta = next_is_lit || n.has_u_in(&["san", "yaN", "Slu", "caN"]);

    if dhatu.text == "pyAy" && next_is_lit_or_yan {
        p.op_term("6.1.29", i, op::text("pI"));
    } else if dhatu.text == "Svi" && next_is_lit_or_yan {
        p.op_optional("6.1.30", op::t(i, op::text("Su")));
    } else if dhatu.text == "hve" && next_will_be_abhyasta {
        p.op_term("6.1.33", i, op::text("hu"));
    }
}

pub fn run_for_abhyasa(p: &mut Prakriya) {
    let i = match p.find_first(T::Abhyasa) {
        Some(i) => i,
        None => return,
    };
    let dhatu = match p.get(i + 1) {
        Some(t) => {
            if t.has_tag(T::Dhatu) {
                t
            } else {
                return;
            }
        }
        None => return,
    };

    let last = p.terms().last().unwrap();

    if last.has_lakshana("li~w") {
        // yadā ca dhātorna bhavati tadā "liṭyabhyāsasya ubhayeṣām"
        // ityabhyāsasya api na bhavati -- kāśikā.
        if is_vaci_svapi(dhatu) && dhatu.text != "Svi" {
            if dhatu.has_u("ve\\Y") {
                p.step("6.1.40")
            } else {
                try_vaci_svapi_samprasarana("6.1.17", p, i)
            }
        } else if is_grahi_jya(dhatu) {
            try_grahi_jya_samprasarana("6.1.17", p, i)
        }
    }
}
