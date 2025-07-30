use super::page::PAGE_SIZE;

// Sizes
pub(crate) const ID_SIZE: usize = 4;
pub(crate) const USERNAME_SIZE: usize = 32;
pub(crate) const EMAIL_SIZE: usize = 255;

// Offsets
pub(crate) const ID_OFFSET: usize = 0;
pub(crate) const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub(crate) const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub(crate) const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub(crate) const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

pub(crate) struct Row {
    pub(crate) id: u32,
    pub(crate) username: String,
    pub(crate) email: String,
}

impl Row {
    pub(crate) fn serialize(self) -> [u8; ROW_SIZE] {
        let id: [u8; ID_SIZE] = self.id.to_le_bytes();
        let username: [u8; USERNAME_SIZE] = self.username.as_bytes().get_n_bytes();
        let email: [u8; EMAIL_SIZE] = self.email.as_bytes().get_n_bytes();

        let mut row = [0u8; ROW_SIZE];
        row[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&id);
        row[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].copy_from_slice(&username);
        row[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&email);
        return row;
    }

    pub(crate) fn deserialize(source: &[u8; ROW_SIZE]) -> Row {
        let id: &[u8; ID_SIZE] = &source[ID_OFFSET..ID_OFFSET + ID_SIZE]
            .try_into()
            .expect("Unable to deserialize id from source");
        let username: &[u8; USERNAME_SIZE] = &source
            [USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]
            .try_into()
            .expect("Unable to deserialize username from source");
        let email: &[u8; EMAIL_SIZE] = &source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE]
            .try_into()
            .expect("Unable to deserialize email from source");
        // TODO: need to remove 0 padding
        Row {
            id: u32::from_le_bytes(id.clone()),
            username: String::from_utf8_lossy(username).to_string(),
            email: String::from_utf8_lossy(email).to_string(),
        }
    }
}

pub(crate) trait NBytes {
    fn get_n_bytes<const N: usize>(&self) -> [u8; N];
}

impl NBytes for &[u8] {
    fn get_n_bytes<const N: usize>(&self) -> [u8; N] {
        let mut result = [0u8; N]; // Create a zero-initialized array
        let len = self.len().min(N); // Determine the number of elements to copy
        result[..len].copy_from_slice(&self[..len]); // Copy the elements
        result
    }
}
