# bogon_filter
A Bogon Filter for removing any addresses that land in RFC3330 space from stdin and sending sanitized list back through  stdout

# Use 
- Take a list of data and send it through | to bogon_filter for filtering:

> echo -e "127.0.0.1\n8.8.8.8\n172.30.5.2\n169.254.254.1\n1.1.1.1\n" | ./bogon_filter

will yield:

> 8.8.8.8
> 1.1.1.1

Works on substring matches so if a private ip is anywhere on the input lines of data, that line will be stripped out.

