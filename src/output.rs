use super::rendering;
use std::net;

/**
 * Something we can output canvases to. Typically this would be a microcontroller connected via USB or wifi.
 */
pub trait RenderTarget {
    fn render_bitmap(&self, bitmap: rendering::BinaryBitmap) -> std::io::Result<()>;
}

pub struct UdpOutput<'a> {
    pub address: &'a str,
}

impl<'a> RenderTarget for UdpOutput<'a> {
    fn render_bitmap(&self, bitmap: rendering::BinaryBitmap) -> std::io::Result<()> {
        let mut buf = Vec::with_capacity(1 + bitmap.buffer.len());
        buf.push(10u8);
        buf.extend(&bitmap.buffer);

        let socket = net::UdpSocket::bind("0.0.0.0:4435").expect("Couldn't bind socket");
        socket.send_to(&buf, self.address)?;
        Ok(())
    }

}