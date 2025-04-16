def test_hash() -> None:
    """Test the hash function for a string."""
    from tsgraph import to_hash_identifier

    string = "test"
    hash_value = to_hash_identifier(string)
    print(hash_value)


def test_summary() -> None:
    """Test the summary function for a tsgraph."""
    from tsgraph import summary_graph

    summary = summary_graph("./tests/data/scannls.tsg")
    print(summary)
