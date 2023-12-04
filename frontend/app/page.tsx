"use client";

import { getCookie } from "cookies-next";
import { useRouter } from "next/navigation";

export default function Home() {
  let authToken = getCookie("authToken");
  let router = useRouter();
  if (authToken) {
    router.push("/movies");
  } else {
    router.push("/signIn");
  }
}
