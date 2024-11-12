import pytest

def test_dataplatform_client():
    try:
        from dataplatform_sdk.config import ClientConfig
        from dataplatform_sdk.clients import DataPlatformClient
        from dataplatform_sdk.credentials import TokenProvider

        my_config = ClientConfig(
            client_name="my-client",
            project="myproject",
            credentials=TokenProvider("somerandomsecrettoken"),
        )

        assert isinstance(my_config, ClientConfig)
        assert my_config.client_name == "my-client"
        assert my_config.project == "myproject"
        assert my_config.timeout == 30
        assert my_config.file_transfer_timeout == 30

        client = DataPlatformClient(my_config)

    except ImportError as e:
        pytest.fail(f"Failed to test 'dataplatform_client': {e}")