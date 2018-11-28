require 'test_helper'

class VisualControllerTest < ActionDispatch::IntegrationTest
  test "should get index" do
    get visual_index_url
    assert_response :success
  end

end
