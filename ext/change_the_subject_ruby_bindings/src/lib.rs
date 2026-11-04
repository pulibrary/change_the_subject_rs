use change_the_subject_rs::fix;
use magnus::{Object, RHash, function, scan_args::get_kwargs};

#[magnus::init(name = "change_the_subject_rs")]
fn init(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("ChangeTheSubjectRS")?;
    module.define_singleton_method("fix", function!(fix_terms, 1))?;
    Ok(())
}

fn fix_terms(arguments: RHash) -> Result<Vec<String>, magnus::Error> {
    let args = get_kwargs::<_, (Vec<String>,), (Option<Vec<String>>,), ()>(
        arguments,
        &["subject_terms"],
        &["separators"],
    )?;
    let subject_terms: Vec<_> = args.required.0.iter().map(|term| term.as_str()).collect();
    let separators = args.optional.0.unwrap_or(vec!["—".to_string()]);
    let separators: Vec<_> = separators.iter().map(|term| term.as_str()).collect();
    Ok(fix(&subject_terms, &separators).collect())
}
