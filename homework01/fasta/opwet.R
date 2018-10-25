fname = "data_without_canon.txt"
x = read.csv(fname, header = F)

p = x[[1]]
fpr = x[[2]]
fnr = x[[3]]

plot(  x = fpr,  y = fnr,  pch = 20,  type = "p",  xlim = c(0, 100),  ylim = c(0, 100))
#text(fpr,     fnr,     labels = p,     cex = 0.7,     pos = 4)
