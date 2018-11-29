class VisualController < ApplicationController
  def index
    @data = ViewerCount.all.order(viewers: :desc)

    respond_to do |format|
      format.html
      format.json { render json: @data }
    end
  end
end
