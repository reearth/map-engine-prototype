import initCore, { InitOutput } from "map-engine-prototype";
import Stats from "stats.js";
import { AxesHelper, PerspectiveCamera, Scene, WebGLRenderer } from "three";

export type Options = {
  target: HTMLElement;
  debug?: boolean;
};

export default class View {
  scene: Scene;
  camera: PerspectiveCamera;
  renderer: WebGLRenderer;

  _options: Options;
  _core: InitOutput | undefined;
  _stats: Stats | undefined;
  _paused = false;

  constructor(options: Options) {
    this._options = options;
    const { target } = options;

    const renderer = new WebGLRenderer({
      antialias: true,
      alpha: true,
    });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(target.offsetWidth, target.offsetHeight);
    target.appendChild(renderer.domElement);

    const axes = new AxesHelper();
    const scene = new Scene();
    scene.add(axes);

    const camera = new PerspectiveCamera(50, target.offsetWidth / target.offsetHeight);
    camera.position.set(1, 1, 1);
    camera.lookAt(scene.position);

    window.addEventListener("resize", () => {
      this.resize(target.offsetWidth, target.offsetHeight);
    });

    this.scene = scene;
    this.camera = camera;
    this.renderer = renderer;

    if (options.debug) {
      this._stats = new Stats();
      target.appendChild(this._stats.dom);
    }
  }

  async init() {
    if (this._core) return;

    const core = await initCore();
    this._core = core;

    this.startMainLoop();
  }

  dispose() {
    this._paused = true;
    this.renderer.dispose();
  }

  startMainLoop() {
    const loop = () => {
      if (this._paused) return;
      this._stats?.begin();

      if (this.update()) this.render();

      this._stats?.end();
      if (!this._paused) requestAnimationFrame(loop);
    };
    requestAnimationFrame(loop);
  }

  resize(width: number, height: number) {
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }

  /** Returns true if the scene was updated and needs to be rendered. */
  update(): boolean {
    return true;
  }

  render() {
    this.renderer.render(this.scene, this.camera);
  }
}
