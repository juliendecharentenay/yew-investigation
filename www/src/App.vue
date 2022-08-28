<template>
<div class="bg-white">
  <div class="max-w-7xl mx-auto px-4 sm:px-6">
    <div class="flex justify-center items-center border-b-2 border-gray-100 py-6">
      <nav class="flex space-x-10">
        <a href="/yew_investigation/vuejs/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> VueJS </a>
        <a href="/yew_investigation/vuejs_wasm/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> VueJS+WebAssembly </a>
        <a href="/yew_investigation/yew/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> Yew </a>
      </nav>
    </div>
  </div>

  <div class="pt-10 pb-10 px-4">
    <div class="relative max-w-7xl mx-auto">
      <div class="text-center">
        <h2 class="text-3xl tracking-tight font-bold text-gray-900 sm:text-4xl">Your choice of implementation:</h2>
        <p class="mt-3 max-w-2xl mx-auto text-xl text-gray-500 sm:mt-4">
        This webpage accompanies the medium story <a href="">Rust on the Web: Yew vs VueJS+WebAssembly</a>. Please select from the 
        3 options below for different implementation of the calculation of a Fibonacci sequence:</p>
      </div>

      <div class="mt-12 max-w-lg mx-auto grid gap-5 lg:grid-cols-3 lg:max-w-none">
        <div class="flex flex-col rounded-lg shadow-lg overflow-hidden">
          <div class="flex-1 bg-white p-6 flex flex-col justify-between">
            <div class="flex-1">
              <p class="text-sm font-medium text-indigo-600">
                <a href="#" class="hover:underline"> JS </a>
              </p>
              <a href="/vuejs/index.html" class="block mt-2">
                <p class="text-xl font-semibold text-gray-900">VueJS</p>
                <p class="mt-3 text-base text-gray-500">An implementation using VueJS in plain javascipt</p>
              </a>
            </div>
          </div>
        </div>

        <div class="flex flex-col rounded-lg shadow-lg overflow-hidden">
          <div class="flex-1 bg-white p-6 flex flex-col justify-between">
            <div class="flex-1">
              <p class="text-sm font-medium text-indigo-600">
                <a href="#" class="hover:underline"> JS/Rust </a>
              </p>
              <a href="#" class="block mt-2">
                <p class="text-xl font-semibold text-gray-900">VueJS & WebAssembly </p>
                <p class="mt-3 text-base text-gray-500">An implementation using VueJS for the rendering elements with the calcuation done in Rust/WebAssembly</p>
              </a>
            </div>
          </div>
        </div>

        <div class="flex flex-col rounded-lg shadow-lg overflow-hidden">
          <div class="flex-1 bg-white p-6 flex flex-col justify-between">
            <div class="flex-1">
              <p class="text-sm font-medium text-indigo-600">
                <a href="#" class="hover:underline"> Rust </a>
              </p>
              <a href="#" class="block mt-2">
                <p class="text-xl font-semibold text-gray-900">Yew</p>
                <p class="mt-3 text-base text-gray-500">A pure Rust implementation using the Yew framework</p>
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<footer class="bg-white border-t-2 border-gray-100 my-12 py-2">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 md:flex md:items-center md:justify-between lg:px-8">
    <div class="mt-8 md:mt-0 md:order-1">
      <p class="text-center text-base text-gray-400">&copy; 2022 Julien de Charentenay</p>
    </div>
  </div>
</footer>
</template>

<script>
export default {
  name: 'App',
  components: { }
}
</script>
