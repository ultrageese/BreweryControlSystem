use egui_dock::{DockArea, Style};

static TABS_CAN_BE_WINDOWS: bool = false;

enum TabTypes{
    Equipment,
    Worker,
    Position,
    Salary,
    Material,
    Recipe,
    Product,
    Order,
    Settings
}
struct Tab{
    tab_type: TabTypes,
    title: String,
}

pub struct App{
    tree: egui_dock::DockState<Tab>,
}

impl Default for App{
    fn default() -> Self{
        let mut tree = egui_dock::DockState::new(vec![
            Tab{
                tab_type: TabTypes::Equipment,
                title:"Оборудование".to_owned(),
            },
            Tab{
                tab_type: TabTypes::Material,
                title: "Сырьё".to_owned(),
            },
            Tab{
                tab_type: TabTypes::Worker,
                title: "Сотрудники".to_owned(),
            }
        ]);

        // let [a,b] = 
        //     tree.main_surface_mut()
        //     .split_left(egui_dock::NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);

        // let [_,_] = tree
        //     .main_surface_mut()
        //     .split_below(a, 0.7, vec!["tab4".to_owned()]);
        // let [_,_] = tree
        //     .main_surface_mut()
        //     .split_below(b, 0.5, vec!["tab5".to_owned()]);
        Self{tree}
    }
}
impl App{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self{
        Default::default()
    }
    
    
}

impl eframe::App for App{
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame){
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer {});
    }
}

struct TabViewer {}
impl TabViewer{
    fn show_equipment(&mut self, ui: &mut egui::Ui){
        ui.label("Equipment");
    }
}

impl egui_dock::TabViewer for TabViewer{
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText{
        (&*tab.title).into()
    }
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab){
        match &tab.tab_type{
            TabTypes::Equipment => {
                &self.show_equipment(ui);
            },
            _ =>{
                ui.label("This is not");
            }
        } 
        
    }
    fn allowed_in_windows(&self, _tab: &mut Self::Tab) -> bool {
        TABS_CAN_BE_WINDOWS
    }
    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        false
    }
}