use rocket::{post, get};
use rocket_dyn_templates::Template;
use std::io::Write;
use rocket::State;
use rocket::form::Form;
use llm::{Model, InferenceParameters};

fn create_gpt3_like_prompt(user_query: &str) -> String {
    format!(
        "I am an AI trained like GPT-3. My purpose is to provide informative, accurate, \
        and helpful responses in a conversational manner. When responding, I use clear, \
        articulate, and engaging language. Here's a question: '{}'\n\nResponse:",
        user_query
    )
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context!{})
}

#[derive(FromForm)]
pub struct UserInput<'r> {
    prompt: &'r str
}

#[post("/ask", data = "<user_input>")]
pub async fn ask(llama: &State<llm::models::Llama>, user_input: Form<UserInput<'_>>) -> String {
    let llama_model = llama.inner();
    let mut session = llama_model.start_session(Default::default());

    let user_input = user_input.into_inner(); // Access the actual UserInput

    let mut output = String::new();

    let res = session.infer::<std::convert::Infallible>(
        llama_model,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: create_gpt3_like_prompt(user_input.prompt).as_str(),
            maximum_token_count: Some(50),
            parameters: Some(&InferenceParameters {
                temperature: 0.7,
                top_k: 50,
                top_p: 0.9,
                ..Default::default()
            }),
            ..Default::default()
        },
        &mut Default::default(),
        |t| {
            output.push_str(t);
            std::io::stdout().flush().unwrap();
            Ok(())
        }
    );

    match res {
        Ok(_) => output, 
        Err(err) => format!("Error during LLaMa inference: {}", err),
    }
}

