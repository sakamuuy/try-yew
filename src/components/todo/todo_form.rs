use yew::{function_component, html, use_state, Callback, Html, InputEvent};

#[function_component(TodoForm)]
pub fn todo_form() -> Html {
    let title = use_state(|| "".to_string());

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();

            match value {
                Some(value) => {
                    title.set((*title).clone() + &value);
                }
                None => {
                    title.set("".to_string());
                }
            }
        })
    };
    html! {
      <form class="mb-5">
        <div class="mb-3">
          <label for="title" class="form-label">{"タイトル"}</label>
          <input type="text" value={(*title).clone()} {oninput} class="form-control" id="title" />
        </div>
        <div class="mb-3">
          {&*title}
        </div>
        <button type="submit" class="btn btn-primary">{"追加"}</button>
      </form>
    }
}
