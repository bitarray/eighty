require "fileutils"

module Processor
  def self.search_description(blocks)
    blocks.each do |b|
      if b.blocks.length != 0
        subblock = Processor.search_description(b.blocks)
        if subblock
          return subblock
        end
      end

      if b.attributes["meta"] == "description"
        return b
      end
    end
    nil
  end

  def self.read_file(path)
    options = {
      :attributes => {
        "outfilesuffix" => ".adoc",
        "relfileprefix" => "@@ASCIIDOCLINK@@:",
        "source-highlighter" => "highlightjs",
        "rouge-css" => "style",
        "stem" => "latexmath",
        "stylesdir" => "/",
        "icons" => "font",
        "idprefix" => "",
        "idseparator" => "-",
        "url" => "@@XREFFULLLINK:#{path}@@@",
        "sectanchors" => true,
      }
    }

    item = {}
    doc = Asciidoctor.load_file path, options

    item[:title] = doc.attributes["doctitle"]
    item[:license] = doc.attributes["license"]
    item[:licenseCode] = doc.attributes["license-code"]
    item[:author] = doc.attributes["author"]
    item[:email] = doc.attributes["email"]
    item[:order] = doc.attributes["order"]
    item[:id] = doc.attributes["id"]
    item[:sitemapTitle] = doc.attributes["sitemap"]
    if item[:order]
      item[:order] = Integer(item[:order])
    end
    item[:toc] = doc.converter.convert(doc, "outline", toclevels: 3)
    item[:created] = doc.attributes["created"]

    description_block = Processor.search_description(doc.blocks)
    item[:description] = description_block.content.tr("\n", " ")

    content = doc.convert
    replaces = []
    content.scan(/@@ASCIIDOCLINK@@:([^#"]*)/).each do |match|
      content_rel_path = match[0]
      content_path = Pathname.new(File.join(File.dirname(path), content_rel_path)).cleanpath

      replace = {
        :from => "@@ASCIIDOCLINK@@:#{content_rel_path}",
        :to => "@@XREFLINK:#{content_path}@@"
      }
      replaces.push(replace)
    end

    replaces.each do |replace|
      content = content.gsub replace[:from], replace[:to]
    end
    item[:content] = content

    Processor.document = item
  end
end
