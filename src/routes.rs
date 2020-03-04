#[get("/total_supply")]
fn total_supply() {

}

#[get("/")]
fn index() -> &'static str {
    "welcome to galaxia api. See endpoints: /total_supply"
}