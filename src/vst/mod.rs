mod editor;
pub mod plugin;
use gui::Window;

use Graphics;
use PluginConfig;
use HostCallback;
use BasePlugin;

#[derive(Default)]
pub struct VSTPlugin<P> where P: BasePlugin + Graphics {
    pub window: Option<Window>,
	pub plugin: P,
	config: PluginConfig,
}
