// Allowed principals
const ALLOWED_PRINCIPALS: [&str; 2] = [
    // Catalyze development principal
    "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae",
    
    // Samer II at Candid UI (https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app)
    "nvifv-62idm-izjcy-rvy63-7tqjz-mg2d7-jiw6m-soqvp-hdayh-mnqf5-yqe",
];

pub fn is_authenticated() -> Result<(), String> {
    let caller = ic_cdk::caller();

    if ALLOWED_PRINCIPALS.contains(&caller.to_string().as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Principal {} is not allowed to call this method",
            caller
        ))
    }
}
