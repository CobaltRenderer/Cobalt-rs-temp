// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use std::sync::Arc;

use bitflags::bitflags;

use crate::{RendererInfo, RendererPlugin};
use crate::renderer::{DeviceType, Feature, GraphicsDevice};
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DeviceEnumerationFlags : i32 {
        const None = sys::Cobalt_DeviceEnumerationFlags_None as i32;
        const HeadlessRendering = sys::Cobalt_DeviceEnumerationFlags_HeadlessRendering as i32;
        const NativeApiValidation = sys::Cobalt_DeviceEnumerationFlags_NativeApiValidation as i32;
    }
}

/// Finds and filters available graphics devices.
///
/// An internal list of devices is kept and is initially empty.
/// `enumerate_devices` will find all available devices.
/// Optional filter methods can then be used to remove devices that do not meet requirements.
/// Devices are then retrieved through `all_devices`, `filtered_devices`
/// and most common `preferred_device` where they can be interrogated further
/// and used to create a [Renderer](crate::renderer::Renderer) object
pub struct GraphicsDeviceEnumerator {
    handle: sys::Cobalt_GraphicsDeviceEnumerator,
    pub(crate) plugin: Arc<RendererPlugin>,
}

impl GraphicsDeviceEnumerator {
    pub(crate) fn new(
        handle: sys::Cobalt_GraphicsDeviceEnumerator,
        flags: sys::Cobalt_DeviceEnumerationFlags,
        plugin: Arc<RendererPlugin>,
    ) -> RendererResult<Self> {
        unsafe {
            return_on_failure!(sys::Cobalt_GraphicsDeviceEnumerator_EnumerateDevices(
                handle, flags,
            ))
        }
        Ok(GraphicsDeviceEnumerator { handle, plugin })
    }

    pub fn preferred_device(&self) -> Option<GraphicsDevice<'_>> {
        let mut device = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_GetPreferredDevice(self.handle, &mut device);
        }
        if device.is_null() {
            None
        } else {
            Some(GraphicsDevice::new(device, self))
        }
    }

    pub fn all_devices(&self) -> Vec<GraphicsDevice<'_>> {
        // We don't know the amount of space required for all devices
        // Allocate what should be enough space and if it fills up
        // We will fetch devices again but with enough space
        let mut capacity: usize = 8;
        let mut devices: Vec<sys::Cobalt_GraphicsDevice> = vec![];
        loop {
            devices.resize(capacity, std::ptr::null_mut());

            let mut length = devices.len();
            unsafe {
                sys::Cobalt_GraphicsDeviceEnumerator_GetAllDevices(
                    self.handle,
                    devices.as_mut_ptr(),
                    &mut length,
                );
            }
            if length > devices.len() {
                capacity = length;
                continue;
            }

            devices.truncate(length);
            return devices
                .iter()
                .map(|x| GraphicsDevice::new(*x, self))
                .collect();
        }
    }

    pub fn filtered_devices(&self) -> Vec<GraphicsDevice<'_>> {
        // We don't know the amount of space required for all devices
        // Allocate what should be enough space and if it fills up
        // We will fetch devices again but with enough space
        let mut capacity: usize = 8;
        let mut devices: Vec<sys::Cobalt_GraphicsDevice> = vec![];
        loop {
            devices.resize(capacity, std::ptr::null_mut());

            let mut length = devices.len();
            unsafe {
                sys::Cobalt_GraphicsDeviceEnumerator_GetFilteredDevices(
                    self.handle,
                    devices.as_mut_ptr(),
                    &mut length,
                );
            }
            if length > devices.len() {
                capacity = length;
                continue;
            }

            devices.truncate(length);
            return devices
                .iter()
                .map(|x| GraphicsDevice::new(*x, self))
                .collect();
        }
    }

    pub fn filter_device(&mut self, target_device: &mut GraphicsDevice) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_FilterDevice(self.handle, target_device.handle);
        }
    }

    pub fn filter_devices_of_type(&mut self, device_type: DeviceType) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_FilterDevicesOfType(
                self.handle,
                device_type as sys::Cobalt_DeviceType,
            );
        }
    }

    pub fn filter_devices_not_of_type(&mut self, device_type: DeviceType) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_FilterDevicesNotOfType(
                self.handle,
                device_type as sys::Cobalt_DeviceType,
            );
        }
    }

    pub fn filter_devices_without_feature(&mut self, feature: Feature) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_FilterDevicesWithoutFeature(
                self.handle,
                feature as sys::Cobalt_Feature,
            );
        }
    }

    pub fn filter_devices_without_all_features(&mut self, features: &[Feature]) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_FilterDevicesWithoutAllFeatures(
                self.handle,
                features.as_ptr() as *const sys::Cobalt_Feature,
                features.len(),
            );
        }
    }

    pub fn clear_device_filters(&mut self) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_ClearDeviceFilters(self.handle);
        }
    }
}

impl Drop for GraphicsDeviceEnumerator {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_GraphicsDeviceEnumerator_Delete(self.handle);
        }
    }
}

unsafe impl Send for GraphicsDeviceEnumerator {}
unsafe impl Sync for GraphicsDeviceEnumerator {}
