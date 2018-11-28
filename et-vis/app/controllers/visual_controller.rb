class VisualController < ApplicationController
  def index

    respond_to do |format|
      format.html
      format.json { render json: [3,5] }
    end
  end
end
