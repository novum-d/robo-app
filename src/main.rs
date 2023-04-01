use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-slate-500">
            <div class="grid place-content-center">
                <div class="form-control">
                  <div class="input-group">
                    <input type="text" placeholder="Search…" class="input input-bordered" />
                    <button class="btn btn-square">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                    </button>
                  </div>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn NormalButton() -> Html {
    html! {
        <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white">{"Save changes"}</button>
    }
}

#[function_component]
pub fn ChatNotification() -> Html {
    html! {
        <div class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4">
            <div class="shrink-0">
                <img class="h-12 w-12" src="assets/img/chat.svg" alt="ChitChat Logo" />
            </div>
            <div>
                <div class="text-xl font-medium text-black">{"ChitChat"}</div>
                <p class="text-slate-500">{"You have a new message!"}</p>
            </div>
        </div>
    }
}

#[function_component]
pub fn ProfileCard() -> Html {
    html! {
        <div class="card w-96 bg-base-100 shadow-xl">
          <figure class="px-10 pt-10">
            <img src="/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" class="rounded-xl" />
          </figure>
          <div class="card-body items-center text-center">
            <h2 class="card-title">{"Shoes!"}</h2>
            <p>{"If a dog chews shoes whose shoes does he choose?"}</p>
            <div class="card-actions">
              <button class="btn btn-primary">{"Buy Now"}</button>
            </div>
          </div>
        </div>
    }
}
