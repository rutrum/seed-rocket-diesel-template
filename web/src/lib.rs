use seed::{prelude::*, *};
use seed_rocket_diesel_template::student::*;

use std::convert::TryInto;

mod api_call;

#[derive(Copy, Clone, Debug)]
enum Error {
    NameRequired,
    FetchError,
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;
        match self {
            NameRequired => "Name field is required",
            FetchError => "Cannot fetch data",
        }.into()
    }
}

#[derive(Clone, Debug, Default)]
struct Model {
    students: Vec<Student>,
    form_err: Option<Error>,
    form_mode: Option<Mode>,
    form: Form,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Create,
    Edit(usize),
}

#[derive(Clone, Debug, Default)]
struct Form {
    name: String,
    gpa: String,
}

impl Form {
    fn reset(&mut self) {
        self.name = String::new();
        self.gpa = String::new();
    }
}

enum FormMsg {
    Name(String),
    Gpa(String),
}

impl Model {
    fn rm_err(&mut self) {
        self.form_err = None;
    } 

    fn set_err(&mut self, e: Error) {
        self.form_err = Some(e);
    } 

    fn err_string(&self) -> String {
        self.form_err.clone()
            .map(|x| x.to_string())
            .unwrap_or_default()
    }
}

enum Msg {
    Fetch,
    Load(Vec<Student>),
    Edit(usize),
    Delete(usize),
    Form(FormMsg),
    Create,
    Submit(Mode),
    Created,
    Error(Error),
    Cancel,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.send_msg(Msg::Fetch);
    Model {
        students: vec![],
        form_err: None,
        ..Model::default()
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders.perform_cmd({
                async move {
                    match api_call::get_students().await {
                        Ok(s) => Msg::Load(s),
                        _ => Msg::Error(Error::NameRequired),
                    }
                }
            });
        }
        Msg::Form(msg) => match msg {
            FormMsg::Name(s) => model.form.name = s,
            FormMsg::Gpa(s) => model.form.gpa = s,
        }
        Msg::Load(students) => {
            model.students = students;
        }
        Msg::Edit(idx) => {
            let student = model.students[idx].clone();
            let gpa = student.gpa.map(|x| x.to_string()).unwrap_or_default();

            model.form.name = student.name;
            model.form.gpa = gpa;

            model.form_mode = Some(Mode::Edit(idx));
        }
        Msg::Delete(idx) => {
            let student = model.students[idx].clone();
            orders.perform_cmd({
                async move {
                    match api_call::delete_student(student).await {
                        Ok(s) if s.status().is_ok() => Msg::Fetch,
                        _ => Msg::Error(Error::NameRequired),
                    }
                }
            });
        }
        Msg::Create => {
            model.form.reset();
            model.form_mode = Some(Mode::Create);
        }
        Msg::Error(e) => {
            model.set_err(e);
        }
        Msg::Created => {
            model.rm_err();
            model.form_mode = None;
            orders.send_msg(Msg::Fetch);
        }
        Msg::Cancel => {
            model.form_mode = None;
        }
        Msg::Submit(mode) => {
            // Pull data from form
            let name = model.form.name.clone();
            let gpa = model.form.gpa.clone();

            match NewStudent::from_strings(name, gpa) {
                Some(new_student) => match mode {
                    Mode::Create => {
                        orders.perform_cmd({
                            async move {
                                match api_call::post_student(new_student).await {
                                    Ok(s) if s.status().is_ok() => Msg::Created,
                                    _ => Msg::Error(Error::NameRequired),
                                }
                            }
                        });
                    }
                    Mode::Edit(idx) => {
                        let id = model.students[idx].id;
                        let student = new_student.with_id(id);
                        orders.perform_cmd({
                            async move {
                                match api_call::put_student(student).await {
                                    Ok(s) if s.status().is_ok() => Msg::Created,
                                    _ => Msg::Error(Error::NameRequired),
                                }
                            }
                        });
                    }
                }
                None => {
                    model.set_err(Error::NameRequired);
                }
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["content"],
        view_students(model),
    ]
}

fn view_students(model: &Model) -> Node<Msg> {
    table![
        tr![
            th![], th!["Name"], th!["GPA"]
        ],
        model.students.iter().enumerate().map(|(idx, student)| {
            let gpa = student.gpa
                .map(|x| format!("{:.1}", x))
                .unwrap_or_default();
            tr![
                td![idx + 1],
                td![&student.name],
                td![gpa],
                td![
                    "edit",
                    ev(Ev::Click, move |_| Msg::Edit(idx)),
                ],
                td![
                    "delete",
                    ev(Ev::Click, move |_| Msg::Delete(idx)),
                ],
            ]
        }),
        if let Some(mode) = model.form_mode {
            view_form(model, mode)
        } else {
            tr![
                td![
                    C!["create-new-button"],
                    "create new",
                    attrs!( At::ColSpan => "3" ),
                    ev(Ev::Click, |_| Msg::Create),
                ],
                td![ style!("visibility"=>"hidden") ],
                td![ style!("visibility"=>"hidden") ],
            ]
        }
    ]
}

fn view_form(model: &Model, mode: Mode) -> Node<Msg> {
    tr![
        C!["form"],
        td![],
        /*
        model.err_string(),
        if let Mode::Edit(idx) = mode {
            format!("Editing student {}", idx + 1)
        } else {
            "Creating new student".into()
        },
        */
        td![
            input![
                input_ev(Ev::Input, |s| Msg::Form(FormMsg::Name(s))),
                attrs!(
                    At::Type => "text",
                    At::Name => "name",
                    At::Value => &model.form.name,
                ),
            ],
        ],
        td![
            input![
                input_ev(Ev::Input, |s| Msg::Form(FormMsg::Gpa(s))),
                attrs!(
                    At::Type => "number",
                    At::Name => "name",
                    At::Value => &model.form.gpa,
                )
            ],
        ],
        button![
            "submit",
            ev(Ev::Click, move |_| Msg::Submit(mode)),
        ],
        button![
            "cancel",
            ev(Ev::Click, move |_| Msg::Cancel),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
