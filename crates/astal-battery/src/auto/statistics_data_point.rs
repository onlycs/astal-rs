// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatisticsDataPoint(Boxed<ffi::AstalBatteryStatisticsDataPoint>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::astal_battery_statistics_data_point_get_type(), ptr as *mut _) as *mut ffi::AstalBatteryStatisticsDataPoint,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::astal_battery_statistics_data_point_get_type(), ptr as *mut _),
        type_ => || ffi::astal_battery_statistics_data_point_get_type(),
    }
}