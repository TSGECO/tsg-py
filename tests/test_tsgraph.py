def test_load():
    from tsgraph import to_hash_identifier

    string = "test"
    hash_value = to_hash_identifier(string)
    print(hash_value)
