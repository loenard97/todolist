use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use log::*;


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, title: &str) -> Self {
        Todo { id, title: title.to_string(), completed: false }
    }
}

#[derive(Properties, PartialEq)]
struct TodoListProps {
    todos: Vec<Todo>,
    on_click: Callback<Todo>,
}

#[function_component(TodoList)]
fn todo_list(TodoListProps { todos, on_click }: &TodoListProps) -> Html {
    let on_click = on_click.clone();

    todos.iter().map(|todo| {
        let mark_as_done = {
            let on_click = on_click.clone();
            let mut todo = todo.clone();
            todo.completed = !todo.completed;
            Callback::from(move |_| {
                on_click.emit(todo.clone())
            })
        };
         
        html! {
            <li key={todo.id} onclick={mark_as_done.clone()}>
                if todo.completed {
                    <s class="decoration-2">{format!("{}", todo.title)}</s>
                } else {
                    {format!("{}", todo.title)}
                }
            </li>
        }
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let todo_json = storage.get_item("todo_vec").unwrap().unwrap_or_default();
    let todo_default_vec = serde_json::from_str(&todo_json).unwrap_or_default();
    
    let todo_state: UseStateHandle<Vec<Todo>> = use_state(|| todo_default_vec);
    let input_state = use_state(|| String::new());
    let id_state = use_state(|| 0);

    let on_click = {
        let todo_state = todo_state.clone();
        Callback::from(move |todo: Todo| {
            let mut todo_vec = todo_state.to_vec();
            let mut id_to_replace = None;
            for (i, val) in todo_vec.iter().enumerate() {
                if val.id == todo.id {
                    id_to_replace = Some(i);
                }
            }
            if id_to_replace.is_some() {
                todo_vec[id_to_replace.unwrap()] = todo;
            }

            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let todo_json = serde_json::to_string(&todo_vec).unwrap();
            let _ = storage.set_item("todo_vec", &todo_json);
            todo_state.set(todo_vec);
        })
    };

    let on_input = {
        let input_state = input_state.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input_state.set(input.value());
            }
        })
    };

    let add_todo = {
        let todo_state = todo_state.clone();
        let input_state = input_state.clone();
        let id_state = id_state.clone();
        Callback::from(move |_| {
            if input_state.is_empty() {
                return
            }
            let mut todo_vec = todo_state.to_vec();
            todo_vec.push(Todo::new(*id_state, &input_state));

            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let todo_json = serde_json::to_string(&todo_vec).unwrap();
            let _ = storage.set_item("todo_vec", &todo_json);
            todo_state.set(todo_vec);

            let id = *id_state + 1;
            id_state.set(id);
        })
    };

    let remove_completed = {
        let todo_state = todo_state.clone();
        Callback::from(move |_| {
            let todo_vec: Vec<Todo> = todo_state.to_vec().iter().cloned()
                .filter(|t| !t.completed)
                .collect();

            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let todo_json = serde_json::to_string(&todo_vec).unwrap();
            let _ = storage.set_item("todo_vec", &todo_json);
            todo_state.set(todo_vec);
        })
    };

    let n_tasks = todo_state.to_vec().len();
    let n_completed = todo_state.to_vec().iter().filter(|t| t.completed).collect::<Vec<&Todo>>().len();

    html! {
    <div class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-400 py-6 sm:py-12">
        <div class="relative bg-white px-6 pt-0 pb-0 shadow-xl ring-1 ring-gray-900/5 sm:mx-auto sm:max-w-lg sm:rounded-lg sm:px-10">
            <div class="mx-auto max-w-md">
                <div class="divide-y divide-gray-300/50">
                    <div class="space-y-6 py-8 text-base leading-7 text-gray-600">
                        <h1 class="text-2xl">{ "Todo list" }</h1>
                        <div>
                            <ul class="list-disc list-inside font-serif">
                                <TodoList todos={todo_state.to_vec()} on_click={on_click.clone()}/>
                            </ul>
                        </div>
                        <hr />
                        <div class="flex space-x-4">
                            <input class="border-2 border-blue-300 rounded-lg p-2" type="text" oninput={on_input} />
                            <button class="bg-blue-300 rounded-lg p-2" onclick={add_todo}>{ "Add todo" }</button>
                        </div>

                        if n_completed > 0 {
                            <div class="flex flex-row">
                                <button class="bg-purple-300 rounded-lg p-2" onclick={remove_completed}>{ "Remove completed todos" }</button>
                            </div>
                        }
                        
                        if n_tasks > 0 {
                            <p>
                                { format!("Completed {n_completed} out of {n_tasks} tasks.") }
                            </p>
                        }
                    </div>
                </div>
            </div>
        </div>
    </div>
    }
}

fn main() {
    let _ = console_log::init_with_level(Level::Debug);
    yew::Renderer::<App>::new().render();
}
