import pytest

def test_retrieve_time_series_by_id(dataplatform_client):
    try:
        result = dataplatform_client.time_series.retrieve(id=1)

        assert result == "{}"

    except ImportError as e:
        pytest.fail(f"Failed to test 'dataplatform_client': {e}")