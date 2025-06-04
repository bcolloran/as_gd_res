extends GdUnitTestSuite

func test_complicated_resource() -> void:
  assert_str("AbcD".to_lower()).is_equal("abcd")
  #  load resource at res://test_complicated_resource.tres
  var res: ComplicatedResource = load("res://test_complicated_resource.tres")
  assert_object(res).is_not_null()
  assert_int(res.value).is_equal(2)
  assert_array(res.int_vec).is_equal([1, 2])