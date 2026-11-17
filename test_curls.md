Mailbox/get

```bash
curl -X POST 'http://192.168.1.33:8787/api' -H 'Content-Type: application/json' -H 'Authorization: Bearer a' -d '{"using": ["urn:ietf:params:jmap:core"], "methodCalls": [["Mailbox/get", {"accountId": "a"}, "dfhg"]]}'; echo ""
```



Email/query

```bash
curl -X POST 'http://192.168.1.33:8787/api' -H 'Content-Type: application/json' -H 'Authorization: Bearer a' -d '{"using": ["urn:ietf:params:jmap:core", "urn:ietf:params:jmap:mail"], "methodCalls": [["Email/query",{"accountId": "a", "calculateTotal": true, "collapseThreads": false, "filter": {"conditions": [{"inMailbox": "a1"}], "operator": "OR"}, "limit": 25, "position": 0, "sort": [{"isAscending": false, "property": "receivedAt"}]}, "256e5dff7"]]}'; echo ""
```




Email/query -> Email/get

```bash
curl -X POST 'http://192.168.1.33:8787/api' -H 'Content-Type: application/json' -H 'Authorization: Bearer a' -d '{"using": ["urn:ietf:params:jmap:core", "urn:ietf:params:jmap:mail"], "methodCalls": [["Email/query",{"accountId": "a", "calculateTotal": true, "collapseThreads": false, "filter": {"conditions": [{"inMailbox": "a1"}], "operator": "OR"}, "limit": 25, "position": 0, "sort": [{"isAscending": false, "property": "receivedAt"}]}, "256e5dff7"], ["Email/get",{"#ids": {"name": "Email/query", "path": "/ids", "resultOf": "256e5dff7"}, "accountId": "a", "bodyProperties": ["partId","blobId", "size", "name", "type", "disposition","cid", "header:Content-Type"], "properties": ["threadId","receivedAt", "sentAt", "keywords", "mailboxIds", "hasAttachment","preview", "headers", "messageId", "inReplyTo", "references"]}, "256e5dff8"]]}'; echo ""
```

