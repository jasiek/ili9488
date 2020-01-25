trait SPIAdapter {
    fn begin(); // TODO: do i *need* this?
    fn end(); // TODO: ^^^

    fn transfer(outgoing: Vec<u8>) -> incoming: Vec<u8>;
}

struct STM32_SPI_Adapter {
}

impl STM32F1_SPIAdapter for SPIAdapter {
    fn transfer(outgoing: Vec<u8>) -> incoming: Vec<u8> {
    }
}
