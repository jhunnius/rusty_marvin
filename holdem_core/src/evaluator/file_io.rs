//! File I/O utilities for poker evaluation tables

use super::errors::EvaluatorError;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Types of lookup tables that can be serialized
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TableType {
    /// 5-card hand lookup table
    FiveCard,
    /// 6-card hand lookup table
    SixCard,
    /// 7-card hand lookup table
    SevenCard,
    /// Flush lookup table
    Flush,
    /// Custom table type
    Custom(u32),
}

impl TableType {
    /// Get the table type ID
    pub fn id(&self) -> u32 {
        match self {
            TableType::FiveCard => 1,
            TableType::SixCard => 2,
            TableType::SevenCard => 3,
            TableType::Flush => 4,
            TableType::Custom(id) => *id,
        }
    }

    /// Create a table type from ID
    pub fn from_id(id: u32) -> Self {
        match id {
            1 => TableType::FiveCard,
            2 => TableType::SixCard,
            3 => TableType::SevenCard,
            4 => TableType::Flush,
            _ => TableType::Custom(id),
        }
    }
}

/// Information about a lookup table
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    /// Type of the table
    pub table_type: TableType,
    /// Version of the table format
    pub version: u32,
    /// Number of entries in the table
    pub entry_count: usize,
    /// Size of each entry in bytes
    pub entry_size: usize,
    /// Creation timestamp
    pub created_at: u64,
    /// Table description
    pub description: String,
}

impl TableInfo {
    /// Create new table info
    pub fn new(table_type: TableType, entry_count: usize, entry_size: usize) -> Self {
        Self {
            table_type,
            version: 1,
            entry_count,
            entry_size,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: String::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Get total table size in bytes
    pub fn total_size(&self) -> usize {
        self.entry_count * self.entry_size
    }
}

/// File manager for lookup tables
pub struct LutFileManager {
    /// Base directory for table files
    base_dir: String,
}

impl LutFileManager {
    /// Create a new file manager with the given base directory
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Save table data to a file
    pub fn save_table<T: AsRef<str>>(
        &self,
        table_type: TableType,
        data: &[u8],
        filename: Option<T>,
    ) -> Result<TableInfo, EvaluatorError> {
        let filename = filename.as_ref().map(|s| s.as_ref()).unwrap_or("table.bin");
        let path = Path::new(&self.base_dir).join(filename);

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        // Write table info header
        let info = TableInfo::new(table_type, data.len(), 1);
        let info_bytes = bincode::serialize(&info)
            .map_err(|e| EvaluatorError::file_io_error(&format!("Serialization error: {}", e)))?;

        writer.write_all(&(info_bytes.len() as u32).to_le_bytes())?;
        writer.write_all(&info_bytes)?;
        writer.write_all(data)?;

        writer.flush()?;

        Ok(info)
    }

    /// Load table data from a file
    pub fn load_table<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(TableInfo, Vec<u8>), EvaluatorError> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        // Read header size
        let mut header_size_bytes = [0u8; 4];
        reader.read_exact(&mut header_size_bytes)?;
        let header_size = u32::from_le_bytes(header_size_bytes) as usize;

        // Read header
        let mut header_bytes = vec![0u8; header_size];
        reader.read_exact(&mut header_bytes)?;
        let info: TableInfo = bincode::deserialize(&header_bytes)
            .map_err(|e| EvaluatorError::file_io_error(&format!("Deserialization error: {}", e)))?;

        // Read data
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok((info, data))
    }

    /// Check if a table file exists
    pub fn table_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    /// Get the full path for a table file
    pub fn get_table_path<P: AsRef<Path>>(&self, filename: P) -> String {
        Path::new(&self.base_dir)
            .join(filename.as_ref())
            .to_string_lossy()
            .to_string()
    }
}

impl Default for LutFileManager {
    fn default() -> Self {
        Self::new("tables")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_table_type() {
        assert_eq!(TableType::FiveCard.id(), 1);
        assert_eq!(TableType::from_id(1), TableType::FiveCard);
        assert_eq!(TableType::Custom(42).id(), 42);
    }

    #[test]
    fn test_table_info() {
        let info = TableInfo::new(TableType::SevenCard, 1000, 4).with_description("Test table");

        assert_eq!(info.table_type, TableType::SevenCard);
        assert_eq!(info.entry_count, 1000);
        assert_eq!(info.entry_size, 4);
        assert_eq!(info.total_size(), 4000);
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_file_manager() {
        let temp_dir = tempdir().unwrap();
        let manager = LutFileManager::new(temp_dir.path());

        let test_data = vec![1u8, 2, 3, 4, 5];
        let info = manager
            .save_table(TableType::FiveCard, &test_data, Some("test.bin"))
            .unwrap();

        assert_eq!(info.entry_count, 5);
        assert_eq!(info.entry_size, 1);

        let (loaded_info, loaded_data) = manager
            .load_table(manager.get_table_path("test.bin"))
            .unwrap();
        assert_eq!(info.table_type, loaded_info.table_type);
        assert_eq!(test_data, loaded_data);
    }
}
