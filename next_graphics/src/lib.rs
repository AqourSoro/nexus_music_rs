

use noa::event::thread_event::{EventData, EventHandlerClient, GlobalEventSender, NexusEventSender, Event, EventType};
use noa::{log::log::*, noa_log, noa_ui_log};
use slint::{SharedString, Weak, LogicalPosition};


slint::include_modules!();




pub fn show_main_window(logger:&'static dyn UILogger, event_sender: &'static dyn EventHandlerClient, main_title:String) ->Result<(), slint::PlatformError>
{
    let result = MainWindow::new();
    let ui = result.and_then(|window|
    {
        noa_ui_log!(logger, LogLevel::INFO("UI is initialized"), stringify!(show_main_window()));
        Ok(window)
    })?;
    
    let _ui_handle = ui.as_weak();

    let _title = SharedString::from(main_title);

    init_ui_callbacks(_ui_handle, logger, event_sender);
    

    ui.invoke_window_init(_title);

    ui.run()

}


fn init_ui_callbacks(ui_handler: Weak<MainWindow>, logger:&'static dyn UILogger, event_sender: &'static dyn EventHandlerClient)
{

    let ui = ui_handler.unwrap();

    let ui_to_close = ui_handler.unwrap();

    let ui_to_move = ui_handler.unwrap().as_weak();



    ui.on_window_init(move|title|
    {
        let ui = ui_handler.unwrap();
        ui.set_app_text(title);

        noa_ui_log!(logger, LogLevel::INFO("Slint UI is initialized"), stringify!(show_main_window(title:SharedString)));
    });

    ui.on_button1_click(move||
    {
        noa_ui_log!(logger, LogLevel::DEBUG("button cliked"), stringify!(ui.on_button1_click()));
        
        event_sender.send_event(Box::new(Event::new(EventType::String, "button 1".to_string().into())));


    });

    ui.on_drag_window(move |offset_x, offset_y|
    {
        let main = ui_to_move.upgrade().unwrap();

        let previous_pos = main.window().position().to_logical(main.window().scale_factor());

        let _new_x = previous_pos.x + offset_x;
        let _new_y = previous_pos.y + offset_y;


        main.window().set_position(LogicalPosition::new(_new_x, _new_y));
        let location_log = format!("Window moved to new location: x: {}, y: {}", _new_x, _new_y);
        noa_ui_log!(logger, LogLevel::TRACE(location_log.as_str()), stringify!(ui.on_drag_window(offset_x:f32, offset_y:f32)));

    });

    ui.on_window_close(move ||
    {
        noa_ui_log!(logger, LogLevel::DEBUG("close window button clicked"), stringify!(ui.on_window_close()));
        ui_to_close.hide().unwrap();
        noa_ui_log!(logger, LogLevel::INFO("Application closed."), stringify!(ui.on_window_close()));
    });

}



pub fn add_in_graphics(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_in_graphics(2, 2);
        assert_eq!(result, 4);
    }
}

