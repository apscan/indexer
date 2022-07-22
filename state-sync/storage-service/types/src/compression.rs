// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::metrics::{
    increment_compression_byte_count, increment_compression_error,
    start_compression_operation_timer, COMPRESS, COMPRESSED_BYTES, DECOMPRESS, RAW_BYTES,
};
use aptos_logger::prelude::*;
use snap::{read, write};
use std::io::{Read, Write};
use thiserror::Error;

/// A wrapper for representing compressed data
pub type CompressedData = Vec<u8>;

/// An error type for capturing compression/decompression failures
#[derive(Clone, Debug, Error)]
#[error("Encountered a compression error! Error: {0}")]
pub struct CompressionError(String);

/// Compresses the data stream using snappy compression.
/// See: https://docs.rs/snap/latest/snap/
pub fn compress_data(raw_data: Vec<u8>) -> Result<CompressedData, CompressionError> {
    // Start the compression timer
    let timer = start_compression_operation_timer(COMPRESS);

    // Compress the data
    let mut encoder = write::FrameEncoder::new(vec![]);
    if let Err(error) = encoder.write_all(&raw_data) {
        increment_compression_error(COMPRESS);
        return Err(CompressionError(format!(
            "Failed to write the data to the encoder: {:?}",
            error.to_string()
        )));
    }
    let compressed_data = match encoder.into_inner() {
        Ok(compressed_data) => compressed_data,
        Err(error) => {
            increment_compression_error(COMPRESS);
            return Err(CompressionError(format!(
                "Failed to fetch the data from the encoder: {:?}",
                error.to_string()
            )));
        }
    };

    // Stop the timer and update the metrics
    let compression_duration = timer.stop_and_record();
    increment_compression_byte_count(RAW_BYTES, raw_data.len() as u64);
    increment_compression_byte_count(COMPRESSED_BYTES, compressed_data.len() as u64);

    // Log the relative data compression statistics
    let relative_data_size = calculate_relative_size(&raw_data, &compressed_data);
    trace!(
        "Compressed {:?} bytes to {:?} bytes ({:?} %) in {:?} seconds.",
        raw_data.len(),
        compressed_data.len(),
        relative_data_size,
        compression_duration
    );
    Ok(compressed_data)
}

/// Decompresses the data stream using snappy decompression
pub fn decompress_data(compressed_data: &CompressedData) -> Result<Vec<u8>, CompressionError> {
    // Start the decompression timer
    let timer = start_compression_operation_timer(DECOMPRESS);

    // Decompress the data
    let mut raw_data = vec![];
    let mut decoder = read::FrameDecoder::new(compressed_data.as_slice());
    if let Err(error) = decoder.read_to_end(&mut raw_data) {
        increment_compression_error(DECOMPRESS);
        return Err(CompressionError(format!(
            "Failed to read the data from the decoder: {:?}",
            error.to_string()
        )));
    };

    // Stop the timer and log the relative data compression statistics
    let decompression_duration = timer.stop_and_record();
    let relative_data_size = calculate_relative_size(compressed_data, &raw_data);
    trace!(
        "Decompressed {:?} bytes to {:?} bytes ({:?} %) in {:?} seconds.",
        compressed_data.len(),
        raw_data.len(),
        relative_data_size,
        decompression_duration
    );
    Ok(raw_data)
}

/// Calculates the relative size (%) between the input and output after a
/// compression/decompression operation, i.e., (output / input) * 100.
fn calculate_relative_size(input: &Vec<u8>, output: &Vec<u8>) -> f64 {
    (output.len() as f64 / input.len() as f64) * 100.0
}
