I ran into an issue with overflow:

```
'attempt to subtract with overflow'
```

I had a u8 that was 0 and I subtracted 1 from it.  

The fix was to use the proper loop variable since I really didn't need to be doing that.