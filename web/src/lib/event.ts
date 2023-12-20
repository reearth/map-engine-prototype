import type { Events } from "map-engine-prototype";
import type { Camera, Scene } from "three";

export function processEvent(_scene: Scene, camera: Camera, event: Events) {
  if (event.camera_transform_updated) {
    const { tx, ty, tz, qx, qy, qz, qw, sx, sy, sz } = event.camera_transform_updated;
    camera.position.set(tx, ty, tz);
    camera.quaternion.set(qx, qy, qz, qw);
    camera.scale.set(sx, sy, sz);
    console.log("camera_transform_updated", tx, ty, tz, qx, qy, qz, qw, sx, sy, sz);
  }
}
