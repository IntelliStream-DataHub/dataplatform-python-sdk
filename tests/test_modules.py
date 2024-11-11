import pytest

def test_import_dataplatform_sdk():
    try:
        from dataplatform_sdk.config import ClientConfig
        from dataplatform_sdk.clients import DataPlatformClient
        from dataplatform_sdk.credentials import CredentialProvider
    except ImportError as e:
        pytest.fail(f"Failed to import 'dataplatform_sdk': {e}")
