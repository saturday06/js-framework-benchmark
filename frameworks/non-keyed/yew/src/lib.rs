#![recursion_limit = "512"]

extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate rand;

use rand::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;

static ADJECTIVES: &[&'static str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&'static str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&'static str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

struct Row {
    id: usize,
    label: String,
}

impl Row {
    fn new(next_id: usize, index: usize) -> Row {
        let mut label = String::new();
        label.push_str(ADJECTIVES[random::<usize>() % ADJECTIVES.len()]);
        label.push(' ');
        label.push_str(COLOURS[random::<usize>() % COLOURS.len()]);
        label.push(' ');
        label.push_str(NOUNS[random::<usize>() % NOUNS.len()]);

        Row {
            id: next_id + index,
            label,
        }
    }
}

pub struct Model {
    rows: Vec<Row>,
    next_id: usize,
    console: ConsoleService,
    selected_id: Option<usize>,
}

pub enum Msg {
    Create(usize),
    Append(usize),
    UpdateEvery(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            rows: Vec::new(),
            next_id: 1,
            console: ConsoleService::new(),
            selected_id: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create(amount) => {
                self.rows = (0..amount)
                    .map(|index| Row::new(self.next_id, index))
                    .collect::<Vec<_>>();
                self.next_id += amount;
                self.console.log(&format!("Create({})", amount));
            }
            Msg::Append(amount) => {
                let next_id = self.next_id;
                self.rows.extend(
                    (0..amount)
                        .map(|index| Row::new(next_id, index))
                        .collect::<Vec<_>>(),
                );
                self.next_id += amount;
                self.console.log(&format!("Append({})", amount));
            }
            Msg::UpdateEvery(every) => {
                for (index, row) in self.rows.iter_mut().enumerate() {
                    if index % every == 0 {
                        row.label += " !!!";
                    }
                }
                self.console.log(&format!("UpdateEvery({})", every));
            }
            Msg::Clear => {
                self.rows.clear();
                self.console.log("Clear");
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
                self.console.log("Swap");
            }
            Msg::Remove(id) => {
                if let Some((index, _)) = self.rows.iter().enumerate().find(|(_, row)| row.id == id)
                {
                    self.rows.remove(index);
                }
                self.console.log(&format!("Remove({})", id));
            }
            Msg::Select(id) => {
                if self.selected_id == Some(id) {
                    self.selected_id = None;
                } else {
                    self.selected_id = Some(id);
                }
                self.console.log(&format!("Select({})", id));
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container",>
                <div class="jumbotron",>
                    <div class="row",>
                        <div class="col-md-6",>
                            <h1>{ "Yew" }</h1>
                        </div>
                        <div class="col-md-6",>
                            <div class="row",>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::Create(1_000), id="run",>{ "Create 1,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::Create(10_000), id="runlots",>{ "Create 10,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::Append(1_000), id="add",>{ "Append 1,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::UpdateEvery(10), id="update",>{ "Update every 10th row" }</button>
                                </div>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::Clear, id="clear",>{ "Clear" }</button>
                                </div>
                                <div class="col-sm-6 smallpad",>
                                    <button type="button", class="btn btn-primary btn-block", onclick=|_| Msg::Swap, id="swaprows",>{ "Swap Rows" }</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <table class="table table-hover table-striped test-data",>
                    <tbody id="tbody",>
                        { for self.rows.iter().map(|row| {
                            let id = row.id.clone();
                            html! {
                                <tr class=if self.selected_id == Some(id) { "danger" } else  { "" },>
                                    <td class="col-md-1",>{ id.to_string() }</td>
                                    <td class="col-md-4", onclick=|_| Msg::Select(id),>
                                        <a class="lbl",>{ row.label.clone() }</a>
                                    </td>
                                    <td class="col-md-1",>
                                        <a class="remove", onclick=|_| Msg::Remove(id),>
                                            <span class="glyphicon glyphicon-remove remove", aria-hidden="true",></span>
                                        </a>
                                    </td>
                                    <td class="col-md-6",></td>
                                </tr>
                            }
                        } ) }
                    </tbody>
                </table>
                <span class="preloadicon glyphicon glyphicon-remove", aria-hidden="true",></span>
            </div>
        }
    }
}
