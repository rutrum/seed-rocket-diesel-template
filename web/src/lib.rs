use seed::{prelude::*, *};

use seed_rocket_diesel_template::student::*;

mod api_call;

enum Error {
    NameRequired,
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;
        match self {
            NameRequired => "Name field is required",
        }.into()
    }
}

struct Model {
    students: Vec<Student>,
    form_err: Option<Error>,
    view_form: bool,
}

enum Msg {
    Reload,
    Create,
    Submit,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        students: vec![],
        form_err: None,
        view_form: false,
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Reload => {

        }
        Msg::Create => {
            model.view_form = true;
        }
        Msg::Submit => {
            model.view_form = false;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        button![
            "create new",
            ev(Ev::Click, |_| Msg::Create),
        ],
        view_students(model),
        if model.view_form {
            view_form(model)
        } else {
            div![]
        }
    ]
}

fn view_students(model: &Model) -> Node<Msg> {
    table![
        th![
            td!["id"], td!["name"], td!["gpa"]
        ],
        model.students.iter().map(|student| {
            let gpa = student.gpa
                .map(|x| x.to_string())
                .unwrap_or_default();
            tr![
                td![student.id],
                td![&student.name],
                td![gpa],
            ]
        })
    ]
}

fn view_form(model: &Model) -> Node<Msg> {
    div![
        div![
            label!["name"],
            input![attrs!(At::Type => "text")],
        ],
        div![
            label!["gpa"],
            input![attrs!(At::Type => "number")],
        ],
        button![
            "submit",
            ev(Ev::Click, |_| Msg::Submit),
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
