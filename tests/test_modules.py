import pytest

def test_import_dataplatform_sdk():
    try:
        from dataplatform_sdk.config import ClientConfig
        from dataplatform_sdk.clients import DataPlatformClient
        from dataplatform_sdk.credentials import CredentialProvider
        from dataplatform_sdk.credentials import TokenProvider
        from dataplatform_sdk.credentials import OAuthClientCredentials
    except ImportError as e:
        pytest.fail(f"Failed to import 'dataplatform_sdk': {e}")

def test_oauthclientcredentials():
    try:
        from dataplatform_sdk.credentials import OAuthClientCredentials

        oauth_client = OAuthClientCredentials(
            token_url="https://keycloak.intellistream.ai",
            client_id="client_id",
            client_secret="client_secret",
            scopes=["openid", "email", "profile"],
        )

        assert oauth_client.token_url == "https://keycloak.intellistream.ai"
        assert oauth_client.client_id == "client_id"
        assert "openid" in oauth_client.scopes
        assert "email" in oauth_client.scopes
        assert "profile" in oauth_client.scopes
        assert len(oauth_client.scopes) == 3

        oauth_client = OAuthClientCredentials(
            token_url="https://keycloak.intellistream.ai",
            client_id="client_id2",
            client_secret="client_secret2"
        )
        assert oauth_client.client_id == "client_id2"
        assert len(oauth_client.scopes) == 0

    except ImportError as e:
        pytest.fail(f"Failed to test 'OAuthClientCredentials': {e}")

def test_oauthclientcredentials():
    try:
        from dataplatform_sdk.credentials import TokenProvider

        oauth_client = TokenProvider(token="somerandomtoken")

        assert oauth_client.token == "somerandomtoken"

    except ImportError as e:
        pytest.fail(f"Failed to test 'OAuthClientCredentials': {e}")