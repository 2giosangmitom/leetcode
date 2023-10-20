import React from "react";
import clsx from "clsx";
import styles from "./styles.module.css";

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: "Beginner friendly",
    description: <>Algorithms is easy to understand, optimal, and accurate.</>,
  },
  {
    title: "Popular programming languages",
    description: (
      <>
        The repository contains solutions to LeetCode problems written in many
        programming languages, such as{" "}
        <b>Ruby, Rust, Go, C#, Dart, JavaScript</b>.
      </>
    ),
  },
  {
    title: "Unit test",
    description: (
      <>You can learn how to write unit tests in many programming languages</>
    ),
  },
];

function Feature({ title, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
