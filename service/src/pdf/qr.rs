use domain::Result;
use qrcode::QrCode;
use qrcode::render::svg;
use tracing::debug;

/// QR code generator for verification links
pub struct QrCodeGenerator;

impl QrCodeGenerator {
    /// Generate QR code as PNG bytes
    pub fn generate_png(data: &str, size: u32) -> Result<Vec<u8>> {
        debug!("Generating QR code for: {}", data);

        let code = QrCode::new(data.as_bytes())
            .map_err(|e| domain::Error::Internal(format!("Failed to create QR code: {}", e)))?;

        // Render as image
        let image = code.render::<image::Luma<u8>>()
            .min_dimensions(size, size)
            .build();

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        use image::ImageEncoder;
        let encoder = image::codecs::png::PngEncoder::new(std::io::Cursor::new(&mut png_bytes));
        encoder
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::L8,
            )
            .map_err(|e| domain::Error::Internal(format!("Failed to encode PNG: {}", e)))?;

        debug!("QR code generated: {} bytes", png_bytes.len());

        Ok(png_bytes)
    }

    /// Generate QR code as SVG string
    pub fn generate_svg(data: &str) -> Result<String> {
        debug!("Generating SVG QR code for: {}", data);

        let code = QrCode::new(data.as_bytes())
            .map_err(|e| domain::Error::Internal(format!("Failed to create QR code: {}", e)))?;

        let svg = code.render::<svg::Color>()
            .min_dimensions(200, 200)
            .build();

        debug!("SVG QR code generated: {} chars", svg.len());

        Ok(svg)
    }

    /// Generate verification QR code for transaction
    pub fn generate_verification_qr(transaction_digest: &str) -> Result<Vec<u8>> {
        let url = format!("https://suiscan.xyz/mainnet/tx/{}", transaction_digest);
        Self::generate_png(&url, 200)
    }

    /// Generate verification QR code as SVG
    pub fn generate_verification_qr_svg(transaction_digest: &str) -> Result<String> {
        let url = format!("https://suiscan.xyz/mainnet/tx/{}", transaction_digest);
        Self::generate_svg(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_png() {
        let data = "https://example.com/verify/0x123";
        let result = QrCodeGenerator::generate_png(data, 200);
        assert!(result.is_ok());

        let png = result.unwrap();
        assert!(!png.is_empty());

        // PNG files start with specific magic bytes
        assert_eq!(&png[0..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn test_generate_svg() {
        let data = "https://example.com/verify";
        let result = QrCodeGenerator::generate_svg(data);
        assert!(result.is_ok());

        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_generate_verification_qr() {
        let tx = "0xabc123def456";
        let result = QrCodeGenerator::generate_verification_qr(tx);
        assert!(result.is_ok());

        let png = result.unwrap();
        assert!(!png.is_empty());
    }
}
