t = -5:0.01:5
x = t.^2 + 2*t
x_neg = (-t).^2 + 2*(-t)
xe = (x + x_neg)/2
xo = (x - x_neg)/2

subplot(3,1,1)
plot(t, x)

subplot(3,1,2)
plot(t, xe)

subplot(3,1,3)
plot(t, xo)
