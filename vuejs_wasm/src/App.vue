<template>
<div class="bg-white">
  <div class="max-w-7xl mx-auto px-4 sm:px-6">
    <div class="flex justify-center items-center border-b-2 border-gray-100 py-6">
      <nav class="flex space-x-10">
        <a href="/vuejs/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> VueJS </a>
        <a href="/vuejs_wasm/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> VueJS+WebAssembly </a>
        <a href="/yew/index.html" class="text-base font-medium text-gray-500 hover:text-gray-900"> Yew </a>
      </nav>
    </div>
  </div>

  <div class="pt-10 pb-10 px-4">
    <div class="relative max-w-7xl mx-auto">
      <div class="text-center">
        <h2 class="text-3xl tracking-tight font-bold text-gray-900 sm:text-4xl">VueJS:</h2>
        <p class="mt-3 max-w-2xl mx-auto text-xl text-gray-500 sm:mt-4">
        Fibonacci sequence implemented in Rust and compiled to WebAssembly:
        </p>

        <button type="button" 
                class="mt-3 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                @click="calculate()"
                id="btn-calculate"
                >
           Calculate
        </button>

        <p class="mt-3 text-gray-500" id="p-result" v-if="result !== null">{{ result }}</p>
        <p class="text-gray-500" v-if="time !== null">Calculated in {{ time }} ms</p>
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
import { fibonacci } from "@/pkg/index.js";

export default {
  name: 'App',
  components: { },
  data: function() {
    return {result: null, time: null};
  },
  methods: {
    calculate: function() {
      const start = performance.now();
      this.result = fibonacci(40);
      const end = performance.now();
      this.time = end - start;
    }
  }
}
</script>
