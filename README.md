# bevy\_pixelperfect

### 2d [Bevy](https://github.com/bevyengine/bevy) plugin that integrates wih the [Pixels](https://github.com/parasyte/pixels) renderer.

The plugin adds a few features to simplify 2d development like an integer 2d `Position` component, a `Scale` and an `Image` component.

| :exclamation:| This is very important   |
|--------------|:-------------------------|
This project is very early and everything will probably change, it is unoptimized as fuck.

## Usage
Add `bevy_pixelperfect` and `bevy` to `Cargo.toml` file:
```toml
[dependencies]
bevy = { version = "0.6.1", default_features = false }
bevy_pixels = "0.1"
```

Add `PixelPerfectPlugin`
```rust
use bevy::prelude:*;
use bevy_pixelperfect::prelued::*;

fn main() {
        App::new()
                .add_plugins(DefaultPlugins)
                .add_plugin(PixelPerfectPlugin)
                ...
                .run();
}
```

## Future Plans
- add Physics system
- add Tilemap lib
- optimize as fuck
