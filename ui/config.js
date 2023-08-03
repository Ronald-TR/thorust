const prodConfig = {
  api: {
    BASE_URL: "/api",
  },
};

const devConfig = {
  api: {
    BASE_URL: "http://localhost:4000/api",
  },
};

module.exports = process.env.NODE_ENV === "production" ? prodConfig : devConfig;
