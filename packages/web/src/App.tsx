import type { Component } from 'solid-js';

import avatar from './assets/avatar.webp';
import Playground from './components/Playground';

const App: Component = () => {
  return (
    <div class="relative w-screen h-screen dark:bg-gray-900 dark:text-white flex flex-col justify-center">
      <Playground />
      <div class="absolute w-screen h-screen bg-transparent flex items-center">
        <div class="container mx-auto">
          <header class="relative px-4 inline-flex bg-inherit items-center space-x-4 py-8 shadow-xl overflow-hidden rounded-md z-10 bg-slate-100/80 dark:bg-slate-700/60">
            <img src={avatar} class="w-32 h-32 md:w-40 md:h-40" />
            <div class="prose dark:prose-invert">
              <h1 class="font-display mb-0 tracking-wide text-transparent bg-clip-text bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-400 inline-block">
                Jaden Giordano
              </h1>
              <p>
                Professional web developer by day; hobbyist game developer by
                night.
              </p>
              <a
                href="https://github.com/greym0uth"
                class="relative no-underline inline-block before:absolute before:top-full before:w-full before:left-0 before:h-[2px] before:bg-gradient-to-r before:from-indigo-500 before:via-purple-500 before:to-pink-400"
              >
                Check out my projects
              </a>
            </div>
          </header>
        </div>
      </div>
    </div>
  );
};

export default App;
