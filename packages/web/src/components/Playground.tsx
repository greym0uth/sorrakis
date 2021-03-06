import { Accessor, Component, createSignal, onMount } from 'solid-js';
import init, { FolioClient } from '@greym0uth/playground';

declare module 'solid-js' {
  namespace JSX {
    interface Directives {
      playground: number;
    }
  }
}

function createPlayground() {
  let canvas: HTMLCanvasElement;

  onMount(() => {
    let root = canvas.parentElement as HTMLDivElement;

    canvas.width = root.offsetWidth;
    canvas.height = root.offsetHeight;

    window.addEventListener('resize', () => {
      canvas.width = root.offsetWidth;
      canvas.height = root.offsetHeight;
    });
  });

  const playground = (ref: HTMLCanvasElement, accessor: Accessor<number>) => {
    canvas = ref;

    init().then(() => {
      const gl = canvas.getContext('webgl');
      const client = new FolioClient(gl!, accessor());

      const render = () => {
          client.update();
          client.render();
          requestAnimationFrame(render);
      };
      requestAnimationFrame(render);
    });
  };

  return playground;
}

const Playground: Component = () => {
  const playground = createPlayground();

  return (
    <div class="w-full h-full box-border">
      <canvas use:playground={Math.round(Math.random() * 3) % 3} width="100%" height="100%" />
    </div>
  );
};

export default Playground;
