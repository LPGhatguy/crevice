# Crevice Changelog

## Unreleased Changes
* Added `AsStd140::std140_size` for easily pre-sizing buffers.
* `Sizer::add` now returns the offset the added value would be written to.
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