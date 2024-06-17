use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
          <div class="mx-auto max-w-screen-md px-6 sm:px-6 lg:px-8 mb-10">
            <div class="flex h-16 items-center justify-between">
              <div class="md:flex md:items-center md:gap-12">
                <a class="block text-gray-700 text-lg hover:text-black" href="#">
                    everdev.it
                </a>
              </div>
              <div class="flex items-center gap-4">
                <div class="flex gap-2">
                    <a href="#" class="block text-gray-600 rounded-md p-2 text-sm font-medium text-black transition hover:bg-gray-100">
                        <Icon width="1.3em" height="1.3em" icon=i::BsTwitterX />
                    </a>
                    <a href="#" class="block text-gray-600 rounded-md p-2 text-sm font-medium text-black transition hover:bg-gray-100">
                        <Icon width="1.3em" height="1.3em" icon=i::FaGithubBrands />
                    </a>
                    <a href="#" class="block text-gray-600 rounded-md p-2 text-sm font-medium text-black transition hover:bg-gray-100">
                        <Icon width="1.3em" height="1.3em" icon=i::BsLinkedin />
                    </a>
                </div>
              </div>
            </div>
          </div>
        </header>
    }
}
