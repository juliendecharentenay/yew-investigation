use yew::prelude::*;

mod fibonacci;
use fibonacci::fibonacci;

enum Msg {
  CalculateFibonacci
}

struct App {
  result: Option<u32>,
  time: Option<f64>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self { Self { result: None, time: None } }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::CalculateFibonacci => {
        println!("Calculate fibonacci");
        let performance = web_sys::window()
            .expect("Should have a window in this context")
            .performance()
            .expect("Performance should be available");
        let start = performance.now();
        self.result = Some(fibonacci(40)); 
        let end = performance.now();
        self.time = Some(end - start); 
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
    let link = ctx.link();
    html! {
<>
  <div class="bg-white">
    <div class="max-w-7xl mx-auto px-4 sm:px-6">
      <div class="flex justify-center items-center border-b-2 border-gray-100 py-6">
        <nav class="flex space-x-10">
          <a href="/yew_investigation/vuejs/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900">{ "VueJS" }</a>
          <a href="/yew_investigation/vuejs_wasm/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900">{ "VueJS+WebAssembly" }</a>
          <a href="/yew_investigation/yew/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900">{ "Yew" }</a>
        </nav>
      </div>
    </div>
  
    <div class="pt-10 pb-10 px-4">
      <div class="relative max-w-7xl mx-auto">
        <div class="text-center">
          <h2 class="text-3xl tracking-tight font-bold text-gray-900 sm:text-4xl">{ "Yew:" }</h2>
          <p class="mt-3 max-w-2xl mx-auto text-xl text-gray-500 sm:mt-4">
          { "Fibonacci sequence implemented in Rust:" }
          </p>
  
          <button type="button" 
                  class="mt-3 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                  onclick={ link.callback(|_| Msg::CalculateFibonacci ) }
                  >
             { "Calculate" }
          </button>

          if let Some(result) = self.result {
            <p class="mt-3 text-gray-500">{ result }</p>
          }
          if let Some(time) = self.time {
            <p class="text-gray-500">{ format!("Calculated in {} ms", time) }</p>
          }
        </div>
      </div>
    </div>
  </div>

  <footer class="bg-white border-t-2 border-gray-100 my-12 py-2">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8">
      <div class="mt-8 md:mt-0 md:order-1">
        <p class="text-center text-base text-gray-400">{ "&copy; 2022 Julien de Charentenay" }</p>
      </div>
    </div>
  </footer>
</>
    }
  }
}

fn main() {
    yew::start_app::<App>();
}
