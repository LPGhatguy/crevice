# Crevice Changelog

## Unreleased Changes

## 0.2.0 (2020-09-22)
* Added documentation for everything in the crate.
* Removed `type_layout` being exposed except for internal tests.
* Fixed alignment offset not taking into account previously added alignment.
* Added `std140::Writer`, for writing dynamically laid out types to buffers.
* Added `std140::Sizer`, for pre-calculating buffer sizes.

## 0.1.0 (2020-09-18)
* Initial MVP release