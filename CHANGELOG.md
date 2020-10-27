# Crevice Changelog

## Unreleased Changes
* Deprecated `Writer::write_slice`, as `Writer::write` now accepts slices.
* Added the `WriteStd140` trait. This trait is more general than `AsStd140` and is automatically implemented for all existing `AsStd140` implementers.
* Added `Writer::write_std140` to write a type that implements `Std140`.
* Changed bounds of some functions, like `Writer::write` to use `WriteStd140` instead of `AsStd140`. This should affect no existing consumers.

## 0.5.0 (2020-10-18)
* Added f64-based std140 types: `DVec2`, `DVec3`, `DVec4`, `DMat2`, `DMat3`, and `DMat4`.
* Added support for std140 structs with alignment greater than 16.
* Fixed padding for std140 matrices; they were previously missing trailing padding.

## 0.4.0 (2020-10-01)
* Added `AsStd140::std140_size` for easily pre-sizing buffers.
* `Writer::write` and `Sizer::add` now return the offset the value is or would be written to.
* Added `std140::DynamicUniform` for aligning dynamic uniform members.
* Added `Writer::write_slice` for writing multiple values in a row.

## 0.3.0 (2020-09-22)
* Added `Std140::as_bytes`, reducing the need to work with bytemuck directly.
* Removed public re-export of bytemuck.

## 0.2.0 (2020-09-22)
* Added documentation for everything in the crate.
* Removed `type_layout` being exposed except for internal tests.
* Fixed alignment offset not taking into account previously added alignment.
* Added `std140::Writer`, for writing dynamically laid out types to buffers.
* Added `std140::Sizer`, for pre-calculating buffer sizes.

## 0.1.0 (2020-09-18)
* Initial MVP release