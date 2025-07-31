"""
ResearchProcess-GPS Adapter Protocol

Defines how external systems (GRAMPS, GEDCOM, FamilySearch, etc.)
integrate with the RGPS engine.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Any, Set, Tuple, IO
from datetime import datetime
from uuid import UUID
from enum import Enum
from dataclasses import dataclass

from .entity_protocol import EntityProtocol, TheoryProtocol


class AdapterCapability(Enum):
    """Capabilities that adapters may support"""
    IMPORT = "import"
    EXPORT = "export"
    SYNC = "sync"
    LIVE_SYNC = "live_sync"
    PARTIAL_IMPORT = "partial_import"
    INCREMENTAL_SYNC = "incremental_sync"
    CONFLICT_DETECTION = "conflict_detection"
    SCHEMA_MAPPING = "schema_mapping"


class ImportMode(Enum):
    """How to handle imports"""
    REPLACE = "replace"  # Replace everything
    MERGE = "merge"      # Merge with existing
    ADDITIVE = "additive"  # Only add new items
    THEORY = "theory"    # Import as new theory


@dataclass
class ImportResult:
    """Result of an import operation"""
    success: bool
    entities_imported: int
    errors: List[str]
    warnings: List[str]
    entity_mapping: Dict[str, UUID]  # External ID -> RGPS ID
    statistics: Dict[str, Any]


@dataclass
class ExportResult:
    """Result of an export operation"""
    success: bool
    entities_exported: int
    errors: List[str]
    warnings: List[str]
    data: Optional[bytes]
    file_path: Optional[str]


@dataclass
class SyncStatus:
    """Status of a sync operation"""
    last_sync: Optional[datetime]
    entities_synced: int
    conflicts: List[Dict[str, Any]]
    pending_changes: int


class AdapterProtocol(ABC):
    """
    Protocol for adapting external genealogy systems to RGPS.
    Each external system (GRAMPS, GEDCOM, etc.) implements this.
    """
    
    @abstractmethod
    def get_name(self) -> str:
        """Return adapter name (e.g., 'GRAMPS', 'GEDCOM7')"""
        pass
    
    @abstractmethod
    def get_version(self) -> str:
        """Return adapter version"""
        pass
    
    @abstractmethod
    def get_capabilities(self) -> Set[AdapterCapability]:
        """Return capabilities this adapter supports"""
        pass
    
    @abstractmethod
    def validate_source(self, source: Any) -> List[str]:
        """
        Validate that the source is readable by this adapter.
        Return list of any issues found.
        """
        pass
    
    # Import Operations
    
    @abstractmethod
    def import_from_file(self, file_path: str, mode: ImportMode = ImportMode.MERGE,
                        options: Optional[Dict[str, Any]] = None) -> ImportResult:
        """Import from a file"""
        pass
    
    @abstractmethod
    def import_from_stream(self, stream: IO, mode: ImportMode = ImportMode.MERGE,
                          options: Optional[Dict[str, Any]] = None) -> ImportResult:
        """Import from a stream"""
        pass
    
    @abstractmethod
    def import_from_api(self, connection_params: Dict[str, Any], 
                       mode: ImportMode = ImportMode.MERGE,
                       options: Optional[Dict[str, Any]] = None) -> ImportResult:
        """Import from an API or database connection"""
        pass
    
    # Export Operations
    
    @abstractmethod
    def export_to_file(self, theory: TheoryProtocol, file_path: str,
                      options: Optional[Dict[str, Any]] = None) -> ExportResult:
        """Export a theory to a file"""
        pass
    
    @abstractmethod
    def export_to_stream(self, theory: TheoryProtocol, stream: IO,
                        options: Optional[Dict[str, Any]] = None) -> ExportResult:
        """Export a theory to a stream"""
        pass
    
    # Mapping Operations
    
    @abstractmethod
    def map_to_rgps(self, external_entity: Any) -> EntityProtocol:
        """Map an external entity to RGPS entity"""
        pass
    
    @abstractmethod
    def map_from_rgps(self, rgps_entity: EntityProtocol) -> Any:
        """Map an RGPS entity to external format"""
        pass
    
    @abstractmethod
    def get_mapping_rules(self) -> Dict[str, Any]:
        """Get the mapping rules for documentation/configuration"""
        pass
    
    # Sync Operations (for adapters that support it)
    
    @abstractmethod
    def setup_sync(self, connection_params: Dict[str, Any]) -> bool:
        """Set up synchronization with external system"""
        pass
    
    @abstractmethod
    def sync_changes(self, since: Optional[datetime] = None) -> SyncStatus:
        """Synchronize changes since last sync"""
        pass
    
    @abstractmethod
    def resolve_conflict(self, conflict: Dict[str, Any], resolution: str) -> bool:
        """Resolve a sync conflict"""
        pass
    
    # Utility Operations
    
    @abstractmethod
    def preview_import(self, source: Any) -> Dict[str, Any]:
        """
        Preview what would be imported without actually importing.
        Returns statistics and sample data.
        """
        pass
    
    @abstractmethod
    def validate_export(self, theory: TheoryProtocol) -> List[str]:
        """
        Validate that a theory can be exported to this format.
        Return list of any issues that would prevent export.
        """
        pass


class GEDCOMAdapter(AdapterProtocol):
    """
    Example: GEDCOM adapter interface.
    Actual implementation would be in adapters/gedcom/
    """
    
    def get_name(self) -> str:
        return "GEDCOM"
    
    def get_capabilities(self) -> Set[AdapterCapability]:
        return {
            AdapterCapability.IMPORT,
            AdapterCapability.EXPORT,
            AdapterCapability.PARTIAL_IMPORT,
            AdapterCapability.SCHEMA_MAPPING
        }
    
    # ... implement all required methods


class GRAMPSAdapter(AdapterProtocol):
    """
    Example: GRAMPS adapter interface.
    Actual implementation would be in adapters/gramps/
    """
    
    def get_name(self) -> str:
        return "GRAMPS"
    
    def get_capabilities(self) -> Set[AdapterCapability]:
        return {
            AdapterCapability.IMPORT,
            AdapterCapability.EXPORT,
            AdapterCapability.SYNC,
            AdapterCapability.LIVE_SYNC,
            AdapterCapability.INCREMENTAL_SYNC,
            AdapterCapability.CONFLICT_DETECTION,
            AdapterCapability.SCHEMA_MAPPING
        }
    
    # ... implement all required methods


class AdapterRegistry:
    """
    Registry for all available adapters.
    This is how the engine discovers and uses adapters.
    """
    
    def __init__(self):
        self._adapters: Dict[str, AdapterProtocol] = {}
    
    def register(self, adapter: AdapterProtocol) -> None:
        """Register an adapter"""
        self._adapters[adapter.get_name()] = adapter
    
    def get_adapter(self, name: str) -> Optional[AdapterProtocol]:
        """Get an adapter by name"""
        return self._adapters.get(name)
    
    def list_adapters(self) -> List[str]:
        """List all registered adapter names"""
        return list(self._adapters.keys())
    
    def get_adapters_with_capability(self, capability: AdapterCapability) -> List[AdapterProtocol]:
        """Get all adapters that support a specific capability"""
        return [
            adapter for adapter in self._adapters.values()
            if capability in adapter.get_capabilities()
        ]