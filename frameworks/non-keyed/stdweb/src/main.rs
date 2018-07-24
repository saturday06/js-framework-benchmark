extern crate rand;
#[macro_use]
extern crate stdweb;

use rand::prng::XorShiftRng;
use rand::{Rng, SeedableRng};
use std::cmp::min;
use std::str::FromStr;
use stdweb::traits::IEvent;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, Element, IElement, IEventTarget, INode, INonElementParentNode};
use stdweb::Reference;

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
    element: Element,
}

impl Row {
    fn generate_label<R>(rng: &mut R) -> String
    where
        R: Rng,
    {
        let mut label = String::new();
        label.push_str(rng.choose(ADJECTIVES).unwrap());
        label.push(' ');
        label.push_str(rng.choose(COLOURS).unwrap());
        label.push(' ');
        label.push_str(rng.choose(NOUNS).unwrap());
        label
    }

    fn td(class_name: &str) -> Element {
        let td = document().create_element("td").expect("create td element");
        td.set_attribute("class", class_name);
        td
    }

    fn get_id(mut element: Element) -> Option<usize> {
        loop {
            if element.node_name() == "TR" {
                if let Some(id) = element.get_attribute("data-id") {
                    return usize::from_str(id.as_ref()).ok();
                }
            }

            if let Some(parent_element) = element.parent_element() {
                element = parent_element;
            } else {
                return None;
            }
        }
    }

    fn new(id: usize, label: String) -> Row {
        let id_string = id.to_string();

        let tr = document().create_element("tr").unwrap();
        tr.set_attribute("data-id", &id_string).unwrap();
        let td1 = td("col-md-1");
        td1.set_text_content(&id_string);
        tr.append_child(&td1);

        let td2 = td("col-md-4");
        tr.append_child(&td2);
        let a2 = document().create_element("a").unwrap();
        a2.set_attribute("class", "lbl").unwrap();
        td2.append_child(&a2);
        a2.set_text_content(&label);

        let td3 = td("col-md-1");
        tr.append_child(&td3);
        let a3 = document().create_element("a").unwrap();
        a3.set_attribute("class", "remove").unwrap();
        td3.append_child(&a3);
        let span = document()
            .create_element("span")
            .expect("create 'span' element");
        span.set_attribute("class", "glyphicon glyphicon-remove remove")
            .unwrap();
        span.set_attribute("aria-hidden", "true").unwrap();
        a3.append_child(&span);

        let td4 = td("col-md-6");
        tr.append_child(&td4);

        Row {
            id,
            label,
            element: tr,
        }
    }

    fn refresh_element(&mut self) {
        let id_string = self.id.to_string();
        self.element.set_attribute("data-id", &id_string);
        self.element
            .child_nodes()
            .item(0)
            .unwrap()
            .set_text_content(&id_string);
        self.refresh_label();
    }

    fn refresh_label(&mut self) {
        self.element
            .child_nodes()
            .item(1)
            .unwrap()
            .child_nodes()
            .item(0)
            .unwrap()
            .set_text_content(self.label.as_ref());
    }
}

struct Main {
    next_id: usize,
    rows: Vec<Row>,
    selected_row_index: Option<usize>,
    rng: XorShiftRng,
    tbody: Element,
}

impl Main {
    fn new() {
        let mut this = Main {
            next_id: 1,
            rows: Vec::new(),
            selected_row_index: None,
            rng: XorShiftRng::from_seed([0; 16]),
            tbody: document().get_element_by_id("tbody").expect("find 'tbody'"),
        };

        document()
            .get_element_by_id("main")
            .expect("get #main element")
            .add_event_listener(move |e: ClickEvent| {
                let target = match e.target() {
                    Some(x) => x,
                    _ => {
                        console!(log, "click event without target");
                        return;
                    }
                };

                let target_element = match Reference::from(target).downcast::<Element>() {
                    Some(x) => x,
                    _ => {
                        console!(log, "click event target is not element");
                        return;
                    }
                };

                if let Some(id) = target_element.get_attribute("id") {
                    match id.as_ref() {
                        "add" => {
                            e.prevent_default();
                            console!(log, "add");
                            this.add();
                            return;
                        }
                        "run" => {
                            e.prevent_default();
                            console!(log, "run");
                            this.run();
                            return;
                        }
                        "update" => {
                            e.prevent_default();
                            console!(log, "update");
                            this.update();
                            return;
                        }
                        "runlots" => {
                            e.prevent_default();
                            console!(log, "runlots");
                            this.runlots();
                            return;
                        }
                        "clear" => {
                            e.prevent_default();
                            console!(log, "clear");
                            this.clear();
                            return;
                        }
                        "swaprows" => {
                            e.prevent_default();
                            console!(log, "swap_rows");
                            this.swaprows();
                            return;
                        }
                        _ => {}
                    }
                }

                let class_list = target_element.class_list();
                if class_list.contains("remove") {
                    e.prevent_default();
                    if let Some(id) = Row::get_id(target_element) {
                        if let Some(index) = this.find_index(id) {
                            console!(log, "delete");
                            this.delete(index);
                            return;
                        }
                    }
                } else if class_list.contains("lbl") {
                    e.prevent_default();
                    if let Some(id) = Row::get_id(target_element) {
                        if let Some(index) = this.find_index(id) {
                            this.select(index);
                            return;
                        }
                    }
                }
            });
    }

    fn find_index(&self, id: usize) -> Option<usize> {
        self.rows
            .iter()
            .enumerate()
            .find(|(_, row)| row.id == id)
            .map(|(index, _)| index)
    }

    fn run(&mut self) {
        self.run_n(1000);
    }

    fn runlots(&mut self) {
        self.run_n(10000);
    }

    fn run_n(&mut self, n: usize) {
        let update_n = min(n, self.rows.len());

        for i in 0..update_n {
            let mut row = &mut self.rows[i];
            row.id = self.next_id + i;
            row.label = Row::generate_label(&mut self.rng);
            row.refresh_element();
        }

        self.next_id += n;

        for i in update_n..n {
            let row = Row::new(self.next_id + i, Row::generate_label(&mut self.rng));
            self.tbody.append_child(&row.element);
            self.rows.push(row);
        }

        self.next_id += (n - update_n);
        self.selected_row_index = None;
    }

    fn add(&mut self) {
        let n = 1000;
        self.rows = (0..n)
            .map(|i| {
                let row = Row::new(self.next_id + i, Row::generate_label(&mut self.rng));
                self.tbody.append_child(&row.element);
                row
            })
            .collect();
        self.next_id += n;
        self.selected_row_index = None;
    }

    fn update(&mut self) {
        let every = 10;
        for i in (0..(self.rows.len() / every)).map(|x| x * every) {
            let mut row = &mut self.rows[i];
            row.label.push_str(" !!!");
            row.refresh_label();
        }
    }

    fn unselect(&mut self) {
        if let Some(index) = self.selected_row_index {
            self.rows[index].element.set_attribute("class", "").unwrap();
            self.selected_row_index = None;
        }
    }

    fn select(&mut self, index: usize) {
        self.unselect();
        let id = self.rows[index].id;
        self.selected_row_index = Some(index);
        self.rows[index]
            .element
            .set_attribute("class", "danger")
            .unwrap();
    }

    fn delete(&mut self, index: usize) {
        for i in  index..(self.rows.len()-1) {
            self.rows[i].label = self.rows[i + 1].label.clone();
            self.rows[i].id = self.rows[i + 1].id;
            self.rows[i].refresh_element();
        }
        self.rows.pop();
    }

    fn clear(&mut self) {
        self.rows.clear();
        self.selected_row_index = None;
        self.tbody.set_text_content("");
    }

    fn swaprows(&mut self) {
        if self.rows.len() <= 998 {
            return;
        }

        let id = self.rows[1].id;
        let label = self.rows[1].label.clone();
        self.rows[1].id = self.rows[998].id;
        self.rows[1].label = self.rows[998].label.clone();
        self.rows[998].id = id;
        self.rows[998].label = label;

        self.rows[1].refresh_element();
        self.rows[998].refresh_element();
    }
}

fn main() {
    Main::new();
}
