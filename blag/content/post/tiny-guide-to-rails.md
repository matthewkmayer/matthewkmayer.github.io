+++
date = "2021-04-13T00:28:53-07:00"
draft = false
title = "Tiny guide to Rails"
+++

## The tiny guide to Rails (that I wish I had)

I've found Rails to rely on more convention over configuration than I'm comfortable with - so much so it seemed impenetrable at times. This is *the tiny guide to Rails I wish I had* to understand where code lives and how the application works.

### Know where to find things

`Go to definition` or `go to implementation` doesn't work well with Rails projects and most IDEs I've tried. It's important to know where to find code:

* Controllers: app/controllers/foo_controller.rb
* Views: app/views/foo/bar.html.erb. Controller app/controllers/**foo_controller.rb** above maps to app/views/**foo**/ and the controller's function `bar` finds the **bar**.html.erb file
* Models: app/models/item_foo.rb

### Models

* Naming: an ActiveRecord model with a declaration like `class ImportFile < ApplicationRecord` will be accessed like `ImportFile.create` but **the file must be named** `import_file.rb`.
* Plurals: if a model called `Widget` has many `ImportFile` models from above, they will be accessed like so: `Widget.find(widget_id).import_files`.
* Fields can be accessed by `.field_name`. For the `ImportFile` class above, if we find an item with `ImportFile.find(import_id)` we can access the `filename` field like so: `ImportFile.find(import_id).filename`. If the field does not exist in the database an error occurs at runtime.
* Lots of validators and attributes etc... are available for Models. Every addition made to the Model file can increase complexity for the next person.
* Schema reference: located in db/schema.rb. This contains most but not all fields available. For example an ID field is in the database but not the schema file. Keep this file up to date for easy reference of field names.

### Controller and View communication

Controllers set instance variables (`@foo`) which are available in the View.

Controller:

```ruby
def show
  @foo = ImportFile.find(import_id)
end
```

View:

```erb
<h1>foo filename is <%= @foo.filename %></h1>
```

If the view tries to access a variable or a field on the variable that doesn't exist an error occurs at runtime.

### Views

Two syntaxes are available in .erb files. 

`<%= RUBY CODE RETURNING A STRING =>` will evaluate the items in the Ruby code block and return the contents.

`<% RUBY CODE %>` will evaluate the contents but not insert the value into the rendered page. This is useful for if/else statements:

```erb
<% if @foo.bar.nil? %>
  <p>❌</p> 
<% else %>
  <p>✅</p> 
<% end %>
```

### Resources

https://guides.rubyonrails.org/
